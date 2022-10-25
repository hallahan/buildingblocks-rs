type Error = Box<dyn std::error::Error>;

fn run() -> Result<(), Error> {
    println!("hello world {:?}", 2);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error", e);
        std::process::exit(1);
    }
}
