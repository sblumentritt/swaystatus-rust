use std::error::Error;
use std::time::Duration;

fn main() {
    let mut handler = ModuleHandler::new();

    handler.add(Box::new(Counter::new()), 1);
    handler.add(Box::new(TwoCounter::new()), 1);
    handler.add(Box::new(Counter::new()), 1);
    handler.add(Box::new(TwoCounter::new()), 1);

    if let Err(e) = handler.run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
struct ModuleWrapper {
    module: Box<dyn Module>,
    interval: u8,
}

struct ModuleHandler {
    modules: Vec<ModuleWrapper>,
}

impl ModuleHandler {
    fn new() -> Self {
        ModuleHandler {
            modules: Vec::new(),
        }
    }

    fn add(&mut self, module: Box<dyn Module>, interval: u8) {
        self.modules.push(ModuleWrapper { module, interval });
    }

    fn buffer(&mut self) -> String {
        let mut buffer = String::new();

        for item in self.modules.iter_mut() {
            item.module.update();

            buffer.push_str(" :: ");
            buffer.push_str(item.module.value().as_str());
        }

        buffer
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            println!("{}", self.buffer());
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

trait Module {
    fn value(&self) -> String;
    fn update(&mut self);
}

struct Counter {
    value: u16,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
}

impl Module for Counter {
    fn value(&self) -> String {
        self.value.to_string()
    }

    fn update(&mut self) {
        self.value += 1;
    }
}

struct TwoCounter {
    value: u64,
}

impl TwoCounter {
    fn new() -> Self {
        TwoCounter { value: 0 }
    }
}

impl Module for TwoCounter {
    fn value(&self) -> String {
        self.value.to_string()
    }

    fn update(&mut self) {
        self.value += 2;
    }
}
