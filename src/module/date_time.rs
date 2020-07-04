use crate::Module;

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
