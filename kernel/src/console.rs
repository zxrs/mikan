use core::fmt;

use share::Global;

use crate::graphics::{PixelColor, PixelWriter};

pub static CONSOLE: Global<Console> = Global::new();

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        if let Some(console) = CONSOLE.inner.borrow_mut().as_mut() {
            _ = write!(console, "{}", format_args!($($args)*));
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        print!("{}\n", format_args!($($args)*));
    };
}

pub fn init(pixel_writer: PixelWriter) {
    let console = Console::new(pixel_writer, PixelColor::BLACK, PixelColor::WHITE);
    *CONSOLE.inner.borrow_mut() = Some(console);
}

#[derive(Debug)]
pub struct Console {
    pixel_writer: PixelWriter,
    bg_color: PixelColor,
    fg_color: PixelColor,
    buffer: [[char; Console::COL as _]; Console::ROW as _],
    cursor_row: u32,
    cursor_col: u32,
}

impl Console {
    const ROW: u32 = 25;
    const COL: u32 = 80;

    pub fn new(pixel_writer: PixelWriter, bg_color: PixelColor, fg_color: PixelColor) -> Self {
        Self {
            pixel_writer,
            bg_color,
            fg_color,
            buffer: [['\0'; _]; _],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn put_string(&mut self, s: &str) -> Option<()> {
        for c in s.chars() {
            if c == '\n' {
                self.new_line()?;
                return Some(());
            }
            if self.cursor_col < Self::COL {
                self.pixel_writer.write_ascii(
                    8 * self.cursor_col,
                    16 * self.cursor_row,
                    c,
                    self.fg_color,
                )?;
                self.buffer[self.cursor_row as usize][self.cursor_col as usize] = c;
                self.cursor_col += 1;
            }
        }
        Some(())
    }

    fn new_line(&mut self) -> Option<()> {
        self.cursor_col = 0;
        if self.cursor_row < Console::ROW - 1 {
            self.cursor_row += 1;
        } else {
            for y in 0..16 * Console::ROW {
                for x in 0..8 * Console::COL {
                    self.pixel_writer.write(x, y, PixelColor::BLACK)?;
                }
            }
            self.buffer.rotate_left(1);
            self.buffer.last_mut()?.fill('\0');
            self.buffer.iter().enumerate().for_each(|(y, a)| {
                a.iter().enumerate().for_each(|(x, c)| {
                    self.pixel_writer.write_ascii(
                        x as u32 * 8,
                        y as u32 * 16,
                        *c,
                        PixelColor::WHITE,
                    );
                });
            });
        }
        Some(())
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_string(s);
        Ok(())
    }
}
