//! Entry point for the dependency-injected version.
//!
//! The whole of main is: collect the command line, choose the production
//! implementations, run the behavior, report whatever it returns. Every
//! decision about what a dependency *is* happens on one line.
//!
//! ```text
//! cargo run --bin injected -- target.txt
//! ```

use domain::dependency_injection::Dependencies;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match Dependencies::from_operating_system(arguments).behavior().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
