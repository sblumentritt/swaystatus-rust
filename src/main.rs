mod module;
use swaystatus::Publisher;

fn main() {
    let mut publisher = Publisher::new();

    publisher.add(Box::new(module::LoadAverage::new()));
    publisher.add(Box::new(module::Memory::new()));
    publisher.add(Box::new(module::DateTime::new()));

    if let Err(e) = publisher.run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
