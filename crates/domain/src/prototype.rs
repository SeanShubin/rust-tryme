//! The prototype: the whole program in one function, talking straight to the
//! operating system.
//!
//! This is the version you write first, when the point is to find out whether
//! the idea works at all. It is short, it has no ceremony, and every line says
//! what it does.
//!
//! It is also untestable, and the reason is structural rather than a matter of
//! effort. `run` reads the command line, the file system and the clock itself,
//! so a test cannot choose what they return. There is no seam to push a fixture
//! through and no return value to assert on -- the output goes to stdout and
//! the elapsed time is whatever the machine happened to take. The only test you
//! can write is one that runs the real program against a real file and reads
//! the real console, which is to say an integration test for a greeting.
//!
//! See [`crate::dependency_injection`] for the same program with the effects
//! moved out to the edge.

use chrono::Utc;
use std::env;
use std::fs;

/// Greets whoever is named in the file given as the first argument, then says
/// how long it took.
///
/// Panics on a missing argument or an unreadable file -- `expect` is the
/// prototype's whole error strategy.
pub fn run() {
    let start_time = Utc::now();
    let file_name = env::args().nth(1).expect("expected a file name argument");
    let target = fs::read_to_string(&file_name).expect("could not read the file");
    println!("Hello, {target}!");
    let duration = Utc::now() - start_time;
    println!("{} milliseconds", duration.num_milliseconds());
}
