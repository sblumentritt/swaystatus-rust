use std::time::Duration;

pub trait Module {
    fn value(&self) -> String;
    fn update(&mut self);
}

pub struct Publisher {
    modules: Vec<Box<dyn Module>>,
}

impl Publisher {
    const SPACER: &'static str = " :: ";

    pub fn new() -> Self {
        Publisher {
            modules: Vec::new(),
        }
    }

    pub fn add(&mut self, module: Box<dyn Module>) {
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

    pub fn run(&mut self) {
        loop {
            self.publish();
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}
