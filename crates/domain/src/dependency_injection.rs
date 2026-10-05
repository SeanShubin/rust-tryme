//! The same program, with every effect moved out to the edge.
//!
//! Nothing here changes what the program does. What changes is who decides.
//! [`ApplicationBehavior`] no longer reaches for the clock, the file system or
//! stdout; it is handed a [`ClockContract`], a [`FilesContract`] and a function
//! to emit with, and uses whatever it was given. The behavior is therefore
//! pure logic over its dependencies, and the tests at the bottom of this file
//! run it against a clock that advances on command and a file system that is a
//! single `Vec<u8>`.
//!
//! The layers, outermost first:
//!
//! - the entry point collects the command line and starts the wiring,
//! - [`Dependencies`] chooses one implementation per contract -- this is the
//!   only place that names a real effect,
//! - [`ApplicationBehavior`] does the work, knowing only the contracts.
//!
//! Compare [`crate::prototype`], which is the same program with those three
//! layers collapsed into one function.

use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

/// What the behavior needs from a clock.
///
/// `DateTime<Utc>` is the canonical moment here, the way `java.time.Instant`
/// is in the Scala original: comparable, orderable, constructible at a chosen
/// moment, and converted to a local time only when something is displayed.
/// Neither of the two clocks in `std::time` would do -- `Instant` cannot be
/// constructed at a chosen moment at all, and `SystemTime` subtracts to an
/// unsigned `Duration`, so asking how far apart two of them are is a `Result`
/// rather than an answer.
pub trait ClockContract {
    fn now(&self) -> DateTime<Utc>;
}

/// What the behavior needs from a file system: one function, the one it calls.
/// A contract is sized by its consumer, not by the thing it stands in for.
pub trait FilesContract {
    fn read_all_bytes(&self, path: &Path) -> io::Result<Vec<u8>>;
}

/// The production clock.
pub struct ClockFromOperatingSystem;

impl ClockContract for ClockFromOperatingSystem {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// The production file system.
pub struct FilesFromOperatingSystem;

impl FilesContract for FilesFromOperatingSystem {
    fn read_all_bytes(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }
}

/// Failures the behavior reports rather than panics on. The prototype's
/// `expect` calls, turned into values a caller can do something about.
#[derive(Debug)]
pub enum ApplicationError {
    MissingFileNameArgument,
    CouldNotReadFile { file_name: String, cause: io::Error },
    FileWasNotUtf8 { file_name: String, cause: FromUtf8Error },
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::MissingFileNameArgument => {
                write!(formatter, "expected a file name argument")
            }
            ApplicationError::CouldNotReadFile { file_name, cause } => {
                write!(formatter, "could not read {file_name}: {cause}")
            }
            ApplicationError::FileWasNotUtf8 { file_name, cause } => {
                write!(formatter, "{file_name} was not utf-8: {cause}")
            }
        }
    }
}

impl std::error::Error for ApplicationError {}

/// The work, expressed against the contracts alone.
///
/// The borrows are what make this cheap to construct in a test: the behavior
/// does not own its dependencies, it points at whatever the composition root
/// is holding. `emit` is a bare `dyn Fn` rather than a trait, mirroring the
/// Scala `String => Unit` -- a dependency with one method does not need a name.
pub struct ApplicationBehavior<'a> {
    arguments: &'a [String],
    clock: &'a dyn ClockContract,
    files: &'a dyn FilesContract,
    emit: &'a dyn Fn(&str),
}

impl<'a> ApplicationBehavior<'a> {
    pub fn new(
        arguments: &'a [String],
        clock: &'a dyn ClockContract,
        files: &'a dyn FilesContract,
        emit: &'a dyn Fn(&str),
    ) -> Self {
        ApplicationBehavior { arguments, clock, files, emit }
    }

    pub fn run(&self) -> Result<(), ApplicationError> {
        let start_time = self.clock.now();
        let file_name = self
            .arguments
            .first()
            .ok_or(ApplicationError::MissingFileNameArgument)?;
        let path = Path::new(file_name);
        let bytes = self.files.read_all_bytes(path).map_err(|cause| {
            ApplicationError::CouldNotReadFile { file_name: file_name.clone(), cause }
        })?;
        let target = String::from_utf8(bytes).map_err(|cause| {
            ApplicationError::FileWasNotUtf8 { file_name: file_name.clone(), cause }
        })?;
        (self.emit)(&format!("Hello, {target}!"));
        let end_time = self.clock.now();
        let duration = end_time - start_time;
        (self.emit)(&format!("{} milliseconds", duration.num_milliseconds()));
        Ok(())
    }
}

/// The composition root: one field per contract, each holding the production
/// implementation. This is the only type in the program that mentions both a
/// contract and a real effect, which is what keeps the choice in one place.
///
/// Substituting a dependency means constructing this differently, so the
/// fields are public and `from_operating_system` is merely the usual choice.
pub struct Dependencies<'a> {
    pub arguments: Vec<String>,
    pub clock: Box<dyn ClockContract + 'a>,
    pub files: Box<dyn FilesContract + 'a>,
    pub emit: Box<dyn Fn(&str) + 'a>,
}

impl<'a> Dependencies<'a> {
    pub fn from_operating_system(arguments: Vec<String>) -> Dependencies<'static> {
        Dependencies {
            arguments,
            clock: Box::new(ClockFromOperatingSystem),
            files: Box::new(FilesFromOperatingSystem),
            emit: Box::new(|line| println!("{line}")),
        }
    }

    /// Hands the behavior borrows of the fields. The returned value cannot
    /// outlive the `Dependencies` it came from, which the lifetime says and the
    /// compiler enforces.
    pub fn behavior(&self) -> ApplicationBehavior<'_> {
        ApplicationBehavior::new(
            &self.arguments,
            self.clock.as_ref(),
            self.files.as_ref(),
            self.emit.as_ref(),
        )
    }
}

/// A clock that hands out a prepared sequence of times, one per call.
///
/// `RefCell` because [`ClockContract::now`] takes `&self` -- the production
/// clock needs no mutation, so forcing `&mut self` on every implementor to suit
/// a test fixture would be the tail wagging the dog. The cost is that the
/// borrow is checked at runtime rather than compile time.
pub struct SequenceClock {
    times: RefCell<Vec<DateTime<Utc>>>,
}

impl SequenceClock {
    /// Builds a clock from milliseconds since the epoch, in the order they
    /// will be handed out.
    pub fn at_millis(millis: &[i64]) -> Self {
        let times = millis
            .iter()
            .rev()
            .map(|value| {
                DateTime::from_timestamp_millis(*value).expect("milliseconds out of range")
            })
            .collect();
        SequenceClock { times: RefCell::new(times) }
    }
}

impl ClockContract for SequenceClock {
    fn now(&self) -> DateTime<Utc> {
        self.times.borrow_mut().pop().expect("the clock ran out of times")
    }
}

/// A file system of exactly one file.
pub struct SingleFile {
    pub name: String,
    pub content: Vec<u8>,
}

impl FilesContract for SingleFile {
    fn read_all_bytes(&self, path: &Path) -> io::Result<Vec<u8>> {
        if path == Path::new(&self.name) {
            Ok(self.content.clone())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no such file: {}", path.display()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collects the lines the behavior emits, so the test asserts on them
    /// instead of on the console.
    fn run_with(
        arguments: &[&str],
        clock: &dyn ClockContract,
        files: &dyn FilesContract,
    ) -> (Result<(), ApplicationError>, Vec<String>) {
        let arguments: Vec<String> = arguments.iter().map(|text| text.to_string()).collect();
        let lines = RefCell::new(Vec::new());
        let emit = |line: &str| lines.borrow_mut().push(line.to_string());
        let result = ApplicationBehavior::new(&arguments, clock, files, &emit).run();
        (result, lines.into_inner())
    }

    #[test]
    fn greets_whoever_the_file_names() {
        let clock = SequenceClock::at_millis(&[1_000, 1_250]);
        let files = SingleFile { name: "target.txt".to_string(), content: b"world".to_vec() };
        let (result, lines) = run_with(&["target.txt"], &clock, &files);
        assert!(result.is_ok());
        assert_eq!(vec!["Hello, world!", "250 milliseconds"], lines);
    }

    /// The elapsed time is an assertion rather than a race, because the clock
    /// is a value the test owns.
    #[test]
    fn elapsed_time_is_whatever_the_clock_says() {
        let clock = SequenceClock::at_millis(&[0, 7]);
        let files = SingleFile { name: "a".to_string(), content: b"you".to_vec() };
        let (_, lines) = run_with(&["a"], &clock, &files);
        assert_eq!("7 milliseconds", lines[1]);
    }

    #[test]
    fn a_missing_argument_is_a_value_not_a_panic() {
        let clock = SequenceClock::at_millis(&[0]);
        let files = SingleFile { name: "a".to_string(), content: b"you".to_vec() };
        let (result, lines) = run_with(&[], &clock, &files);
        assert_eq!(
            "expected a file name argument",
            result.unwrap_err().to_string()
        );
        assert!(lines.is_empty());
    }

    #[test]
    fn an_unreadable_file_is_reported_with_its_name() {
        let clock = SequenceClock::at_millis(&[0]);
        let files = SingleFile { name: "present.txt".to_string(), content: b"you".to_vec() };
        let (result, _) = run_with(&["absent.txt"], &clock, &files);
        assert_eq!(
            "could not read absent.txt: no such file: absent.txt",
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn bytes_that_are_not_utf_8_are_reported_rather_than_replaced() {
        let clock = SequenceClock::at_millis(&[0]);
        let files = SingleFile { name: "a".to_string(), content: vec![0xff, 0xfe] };
        let (result, _) = run_with(&["a"], &clock, &files);
        assert!(result.unwrap_err().to_string().starts_with("a was not utf-8:"));
    }

    /// The production wiring is itself a value, so a test can take it apart and
    /// replace one field without the behavior noticing.
    #[test]
    fn the_composition_root_can_be_rewired_one_field_at_a_time() {
        let lines = RefCell::new(Vec::new());
        // Scoped so the wiring -- and with it the borrow of `lines` -- is gone
        // before the assertion takes ownership of what was collected.
        {
            let dependencies = Dependencies {
                arguments: vec!["a".to_string()],
                clock: Box::new(SequenceClock::at_millis(&[0, 3])),
                files: Box::new(SingleFile { name: "a".to_string(), content: b"there".to_vec() }),
                emit: Box::new(|line: &str| lines.borrow_mut().push(line.to_string())),
            };
            assert!(dependencies.behavior().run().is_ok());
        }
        assert_eq!(vec!["Hello, there!", "3 milliseconds"], lines.into_inner());
    }
}
