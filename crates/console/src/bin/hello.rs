use chrono::{Local, SecondsFormat, Utc};

fn main() {
    let start_time = Utc::now();
    println!("Hello, world!");
    println!("{}", start_time.to_rfc3339());
    println!("{}", start_time.to_rfc3339_opts(SecondsFormat::Millis, true));
    println!("{}", start_time.to_rfc3339_opts(SecondsFormat::Millis, false));
    println!("local {}", start_time.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S"));
    let end_time = Utc::now();
    let duration = end_time - start_time;
    println!("took  {} microseconds", duration.num_microseconds().unwrap());
}
