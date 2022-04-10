use core::ops::{Deref, DerefMut};

use super::color::ColorPalette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_palette: ColorPalette,
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl Deref for ScreenChar {
    type Target = ScreenChar;

    fn deref(&self) -> &Self::Target {
        self
    }
}
