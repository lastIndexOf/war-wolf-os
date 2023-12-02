pub mod writer;

pub use writer::*;

#[allow(dead_code)]
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
    ascii_code: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
