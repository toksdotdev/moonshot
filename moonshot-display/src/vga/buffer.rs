use volatile::Volatile;

use super::character::ColoredCharacter;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ColoredCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
