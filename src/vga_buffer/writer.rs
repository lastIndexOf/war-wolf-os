use super::{Buffer, ColorCode, ScreenChar};

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

#[repr(C)]
pub struct Writer {
    pub col_pos: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let y = BUFFER_HEIGHT - 1;
                let x = self.col_pos;

                let word = ScreenChar {
                    ascii_code: byte,
                    color_code: self.color_code,
                };

                self.buffer.chars[y][x] = word;
                self.col_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        todo!()
    }
}
