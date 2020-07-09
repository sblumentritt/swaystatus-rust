#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod module;

fn main() {
    let mut publisher = swaystatus::Publisher::default();

    publisher.add(Box::new(module::Updates::new()));
    publisher.add(Box::new(module::LoadAverage::new()));
    publisher.add(Box::new(module::Memory::new()));
    publisher.add(Box::new(module::Clock::new()));

    publisher.run();
}
