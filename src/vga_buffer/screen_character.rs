
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black =         0x0,
    Blue =          0x1,
    Green =         0x2,
    Cyan =          0x3,
    Red =           0x4,
    Magenta =       0x5,
    Brown =         0x6,
    LightGray =     0x7,
    DarkGray =      0x8,
    LightBlue =     0x9,
    LightGreen =    0xa,
    LightCyan =     0xb,
    LightRed =      0xc,
    Pink =          0xd,
    Yellow =        0xe,
    White =         0xf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}
