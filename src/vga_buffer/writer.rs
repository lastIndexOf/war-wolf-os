use core::{
    fmt::{self, Write},
    ptr::write_volatile,
};

use lazy_static::lazy_static;
use spin::Mutex;

use super::{Buffer, Color, ColorCode, ScreenChar};

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color_code: ColorCode::new(Color::Yellow as u8, Color::Black as u8),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[repr(C)]
pub struct Writer {
    pub col_pos: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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

                // 易失性操作，确保编译器不会将该读写操作优化掉
                unsafe {
                    write_volatile(&mut self.buffer.chars[y][x] as *mut _, word);
                };

                self.col_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // UTF-8 编码特点：
                // 如果一个字符占用多个字节，那么每个组成它的独立字节都不是有效的 ASCII 码字节
                // 0x20 - 0x7e 是可打印的 ASCII 字符，其余的 ASCII 编码都是控制字符，不可打印
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 0xfe 在 VGA 硬件里被编码为“■”
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                unsafe {
                    write_volatile(
                        &mut self.buffer.chars[y - 1][x] as *mut _,
                        self.buffer.chars[y][x],
                    );
                };
            }
        }

        self.clear_col(BUFFER_HEIGHT - 1);

        self.col_pos = 0;
    }

    fn clear_col(&mut self, y: usize) {
        let blank = ScreenChar {
            ascii_code: b' ',
            color_code: self.color_code,
        };

        for x in 0..BUFFER_WIDTH {
            unsafe {
                write_volatile(&mut self.buffer.chars[y][x] as *mut _, blank);
            };
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
