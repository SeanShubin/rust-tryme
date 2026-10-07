use std::fs;
use std::process::ExitCode;
use chrono::Utc;
use std::env::args;
use std::error::Error;

fn main() -> ExitCode {
    match run(args()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let start_time = Utc::now();
    let file_name = args.skip(1).next().ok_or("usage: hello1 <filename>")?;
    let target = fs::read_to_string(&file_name)?;
    println!("Hello, {target}!");
    let end_time = Utc::now();
    let duration = end_time - start_time;
    let duration_microseconds = duration.num_microseconds().ok_or("duration overflow")?;
    let limit_microseconds = 100;
    println!("took {duration_microseconds} microseconds");
    if duration_microseconds > limit_microseconds {
        Err(format!("limit is {limit_microseconds} microseconds").into())
    } else {
        Ok(())
    }
}
