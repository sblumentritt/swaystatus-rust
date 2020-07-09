// load the modules from the external files e.g. module/clock.rs as private
// and bring the structs into this 'mod' which abstracts the file hierarchy

mod clock;
pub use self::clock::Clock;

mod load_average;
pub use self::load_average::LoadAverage;

mod memory;
pub use self::memory::Memory;

mod archlinux;
pub use self::archlinux::Updates;
