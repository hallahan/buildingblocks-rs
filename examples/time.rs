extern crate chrono;
use chrono::{TimeZone, Utc};

type Error = Box<dyn std::error::Error>;

fn run() -> Result<(), Error> {
    let d = Utc::now().signed_duration_since(Utc.datetime_from_str("Sep 01 00:00:01 1993", "%b %d %H:%M:%S %Y")?).num_days();
    println!("Today is: Sept {} 1993", d);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
}
