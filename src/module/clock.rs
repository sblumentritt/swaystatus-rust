use swaystatus::Module;

pub struct Clock {
    value: chrono::DateTime<chrono::Local>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            value: chrono::Local::now(),
        }
    }
}

impl Module for Clock {
    fn value(&self) -> String {
        // TODO: find a way to change AM/PM to Japanese
        self.value.format("%I:%M %p").to_string()
    }

    fn update(&mut self) {
        self.value = chrono::Local::now();
    }
}
