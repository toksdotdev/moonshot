use core::ops::Deref;
use core::ops::DerefMut;

use super::color::ColorPalette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ColoredCharacter {
    pub ascii_character: u8,
    pub color_palette: ColorPalette,
}

impl DerefMut for ColoredCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl Deref for ColoredCharacter {
    type Target = ColoredCharacter;

    fn deref(&self) -> &Self::Target {
        self
    }
}
