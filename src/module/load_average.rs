use swaystatus::Module;

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
        let cpu_count = match fs::read_to_string(Self::CPUINFO_PATH) {
            #[allow(clippy::cast_possible_truncation)]
            Ok(content) => content
                .lines()
                .filter(|line| line.contains("model name"))
                .count() as u8,
            Err(err) => {
                eprintln!("Error reading cpuinfo: {}", err);

                // just use a default value as the program should still continue
                0
            }
        };

        Self {
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
            "{:.2} {:.2} {:.2} ({})",
            self.one, self.five, self.fiftteen, self.cpu_count
        )
    }

    fn update(&mut self) {
        let loadavg = match fs::read_to_string(Self::LOADAVG_PATH) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading loadavg: {}", err);
                return;
            }
        };

        let mut array: [f32; 3] = [0.0; 3];

        for (index, split) in loadavg.split_whitespace().enumerate() {
            if index <= 2 {
                array[index] = match split.parse::<f32>() {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("Problem parsing value as f32: {}", err);
                        -1.0
                    }
                };
            } else {
                // stop the for-loop as no more data is needed
                break;
            }
        }

        if !array.contains(&-1.0) {
            self.one = array[0];
            self.five = array[1];
            self.fiftteen = array[2];
        }
    }
}
