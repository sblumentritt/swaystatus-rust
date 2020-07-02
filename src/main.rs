use std::error::Error;
use std::time::Duration;

fn main() {
    let mut publisher = Publisher::new();

    publisher.add(Box::new(module::LoadAverage::new()));
    publisher.add(Box::new(module::DateTime::new()));

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

mod module {
    use super::Module;

    pub struct DateTime {
        value: chrono::DateTime<chrono::Local>,
    }

    impl DateTime {
        pub fn new() -> Self {
            DateTime {
                value: chrono::Local::now(),
            }
        }
    }

    impl Module for DateTime {
        fn value(&self) -> String {
            // TODO: find a way to change AM/PM to Japanese
            self.value.format("%I:%M %p").to_string()
        }

        fn update(&mut self) {
            self.value = chrono::Local::now();
        }
    }

    use std::fs;

    pub struct LoadAverage {
        one: f32,
        five: f32,
        fiftteen: f32,
        cpu_count: u8,
    }

    impl LoadAverage {
        const LOADAVG_PATH: &'static str = "/proc/loadavg";
        const CPUINFO_PATH: &'static str = "/proc/cpuinfo";

        pub fn new() -> Self {
            // TODO: replace unwrap() calls with correct error handling
            let cpu_count = fs::read_to_string(LoadAverage::CPUINFO_PATH)
                .unwrap()
                .lines()
                .filter(|line| line.contains("model name"))
                .collect::<Vec<_>>()
                .len() as u8;

            LoadAverage {
                one: 0.0,
                five: 0.0,
                fiftteen: 0.0,
                cpu_count,
            }
        }
    }

    impl Module for LoadAverage {
        fn value(&self) -> String {
            format!(
                "{} {} {} ({})",
                self.one, self.five, self.fiftteen, self.cpu_count
            )
        }

        fn update(&mut self) {
            // TODO: replace unwrap() calls with correct error handling
            let loadavg = fs::read_to_string(LoadAverage::LOADAVG_PATH).unwrap();

            for (index, split) in loadavg.split_whitespace().enumerate() {
                if index == 0 {
                    self.one = split.parse::<f32>().unwrap();
                } else if index == 1 {
                    self.five = split.parse::<f32>().unwrap();
                } else if index == 2 {
                    self.fiftteen = split.parse::<f32>().unwrap();

                    // stop the for-loop as no more data is needed
                    break;
                }
            }
        }
    }
}
