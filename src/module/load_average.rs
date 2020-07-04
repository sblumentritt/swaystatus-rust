use crate::Module;
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
