use super::{
    buffer::{Buffer, BUFFER_HEIGHT, BUFFER_WIDTH},
    character::ScreenChar,
    color::{Color, ColorPalette},
};
use core::fmt;

pub struct Writer {
    pub column_position: usize,
    pub color_palette: ColorPalette,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn reset_color(&mut self) {
        self.set_color_palette(Self::default().color_palette);
    }

    pub fn set_color_palette(&mut self, palette: ColorPalette) {
        self.color_palette = palette;
    }

    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let (row, col) = (BUFFER_HEIGHT - 1, self.column_position);
                self.buffer.chars[row][col].write(ScreenChar {
                    color_palette: self.color_palette,
                    ascii_character: byte,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_palette: self.color_palette,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Default for Writer {
    fn default() -> Self {
        Writer {
            column_position: 0,
            color_palette: ColorPalette::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
}
