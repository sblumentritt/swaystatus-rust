mod module;

fn main() {
    let mut publisher = swaystatus::Publisher::new();

    publisher.add(Box::new(module::LoadAverage::new()));
    publisher.add(Box::new(module::Memory::new()));
    publisher.add(Box::new(module::Clock::new()));

    if let Err(e) = publisher.run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
