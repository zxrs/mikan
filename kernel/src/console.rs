use crate::graphics::{PixelColor, PixelWriter};

#[derive(Debug)]
pub struct Console<'a> {
    pixel_writer: &'a mut PixelWriter,
    bg_color: PixelColor,
    fg_color: PixelColor,
    buffer: [[char; Console::COL as _]; Console::ROW as _],
    cursor_row: u32,
    cursor_col: u32,
}

impl<'a> Console<'a> {
    const ROW: u32 = 25;
    const COL: u32 = 80;

    pub fn new(
        pixel_writer: &'a mut PixelWriter,
        bg_color: PixelColor,
        fg_color: PixelColor,
    ) -> Self {
        Self {
            pixel_writer,
            bg_color,
            fg_color,
            buffer: [[0u8 as _; _]; _],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn put_string(&mut self, s: &str) -> Option<()> {
        for c in s.chars() {
            if c == '\n' {
                self.new_line();
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

    fn new_line(&self) {}
}
