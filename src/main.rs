use std::error::Error;
use std::time::Duration;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut count = 0;

    loop {
        println!("{}", count);
        count += 1;
        std::thread::sleep(Duration::from_secs(1));
    }
}
