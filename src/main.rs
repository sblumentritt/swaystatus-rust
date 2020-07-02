use std::error::Error;
use std::time::Duration;

fn main() {
    let mut publisher = Publisher::new();

    if let Err(e) = publisher.run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

struct Publisher {
    modules: Vec<Box<dyn Module>>,
}

impl Publisher {
    const SPACER: &'static str = " :: ";

    fn new() -> Self {
        Publisher {
            modules: Vec::new(),
        }
    }

    fn add(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    fn publish(&mut self) {
        let mut buffer = String::new();

        for (index, item) in self.modules.iter_mut().enumerate() {
            item.update();

            // the left side of the buffer (first element) should not start with spacer
            if index != 0 {
                buffer.push_str(Publisher::SPACER);
            }

            buffer.push_str(item.value().as_str());
        }

        println!("{}", buffer);
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.publish();
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}

trait Module {
    fn value(&self) -> String;
    fn update(&mut self);
}
