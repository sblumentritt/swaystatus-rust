use std::time::Duration;

/// Functionality which a module needs to implement.
pub trait Module {
    /// Returns a string which will be used as is.
    fn value(&self) -> String;
    /// Updates the internal state of the object.
    fn update(&mut self);
}

/// Holds a list of modules.
#[derive(Default)]
pub struct Publisher {
    modules: Vec<Box<dyn Module>>,
}

impl Publisher {
    /// Spacer which will be used between modules values.
    const SPACER: &'static str = " :: ";

    /// Adds a module to the list
    ///
    /// # Arguments
    ///
    /// * `module` - An object which implements the Module trait.
    pub fn add(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    /// Creates a string from all the modules and prints it on stdout.
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

        println!("{} ", buffer);
    }

    /// Main loop which publishes the data every 2 seconds.
    pub fn run(&mut self) {
        loop {
            self.publish();
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}
