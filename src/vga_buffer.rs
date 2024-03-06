pub mod writer;

pub use writer::*;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: u8, background: u8) -> Self {
        ColorCode(foreground << 4 | background)
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_code: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn write_byte(&mut self, x: usize, y: usize, byte: u8, color: ColorCode) {
        self.chars[y][x] = ScreenChar {
            ascii_code: byte,
            color_code: color,
        };
    }
}
