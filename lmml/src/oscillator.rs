pub use saw::*;
pub use sine::*;
pub use square::*;
pub use triangle::*;

mod saw;
mod sine;
mod square;
mod triangle;

pub const SAMPLE_RATE: u32 = 44100;
