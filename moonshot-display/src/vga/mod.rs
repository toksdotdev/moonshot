use self::color::ColorPalette;
use crate::vga::writer::Writer;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

mod buffer;
mod character;
mod color;
mod writer;

lazy_static! {
    static ref VGA_DISPLAY: Mutex<Writer> = Mutex::new(Writer::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)))
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| VGA_DISPLAY.lock().write_fmt(args).unwrap());
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut writer = VGA_DISPLAY.lock();
        writer.set_color_palette(ColorPalette::error());
        writer.write_fmt(args).unwrap();
        writer.reset_color();
    });
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::vga::_eprint(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ({
        $crate::eprint!("{}\n", format_args!($($arg)*));
    })
}

#[cfg(test)]
mod tests {
    use crate::vga::buffer::BUFFER_HEIGHT;
    use crate::vga::VGA_DISPLAY;

    #[test_case]
    fn test_println_doesnt_panic() {
        for _ in 0..200 {
            println!("printing text to screen!");
        }
    }

    #[test_case]
    fn test_println_output_is_correct() {
        let s = "Some very random string on a single line";
        println!("{}", s);

        for (i, c) in s.chars().enumerate() {
            let colored_char = VGA_DISPLAY.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(colored_char.ascii_character), c);
        }
    }

    #[test_case]
    fn test_println_wraps_text_longer_than_screen_width() {
        let s = "Some very random text that exceeds the total length of characters expected for a single line on a VGA buffer screen";
        println!("{}", s);

        let (line_3, line_2) = s.split_at(80);
        for (column, t_char) in line_3.chars().enumerate() {
            let c_char = VGA_DISPLAY.lock().buffer.chars[BUFFER_HEIGHT - 3][column].read();
            assert_eq!(char::from(c_char.ascii_character), t_char);
        }

        for (column, t_char) in line_2.chars().enumerate() {
            let c_char = VGA_DISPLAY.lock().buffer.chars[BUFFER_HEIGHT - 2][column].read();
            assert_eq!(char::from(c_char.ascii_character), t_char);
        }
    }

    #[test_case]
    fn test_new_lines() {
        let texts = ["line A", "line B", "line C"];
        for text in texts {
            println!("{}", text);
        }

        for (line, text) in texts.iter().rev().enumerate() {
            for (column, t_char) in text.chars().enumerate() {
                let c_char =
                    VGA_DISPLAY.lock().buffer.chars[BUFFER_HEIGHT - line - 2][column].read();
                assert_eq!(char::from(c_char.ascii_character), t_char);
            }
        }
    }
}
