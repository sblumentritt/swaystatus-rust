// NOTE: is this really the best way?

mod clock;
pub use self::clock::Clock;

mod load_average;
pub use self::load_average::LoadAverage;

mod memory;
pub use self::memory::Memory;
