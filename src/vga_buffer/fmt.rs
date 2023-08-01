use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::vga_buffer::{Color, ColorCode, ScreenChar};

use std::string::String;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};
use crate::buffer::{Buffer, infinite};
use crate::buffer::infinite::InfiniteBuffer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor: (0, 0),
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer_window: unsafe { &mut *(0xb8000 as *mut BufferWindow) },
        buffer: InfiniteBuffer::new(0, 0)
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct BufferWindow {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    cursor: (usize, usize),
    color_code: ColorCode,
    buffer_window: &'static mut BufferWindow,
    buffer: InfiniteBuffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                self.buffer.write(byte as char);
                self.focus();
            }
        }
    }

    pub fn terminal_set_cursor(&mut self)
    {
        let screen_curs = self.cursor.clone();
        let buff_curs = self.buffer.cursor.clone();

        let y = (buff_curs.0 - screen_curs.0) as u16;
        let x = (buff_curs.1 - screen_curs.1) as u16;

        let pos = y * (BUFFER_WIDTH as u16) + x;
        let mut port_inst: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3D4);
        let mut port_val: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3D5);

        unsafe {
            port_inst.write(0x0F);
            port_val.write((pos.clone() & 0xFF) as u8);
            port_inst.write(0x0E);
            port_val.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn delete_byte(&mut self) {
        self.buffer.delete_char();
        self.focus();
    }

    pub fn get_line(&self) -> String {
        self.buffer.read_cursor_line().expect("Unable to get line.")
    }

    fn new_line(&mut self) {
        self.buffer.write_line();
        self.focus();
    }

    fn move_cursor(&mut self, dx: usize, dy: usize) {
        self.cursor.0 += dx;
        self.cursor.1 += dy;
    }

    fn move_cursor_to(&mut self, x: usize, y: usize) {
        self.cursor.0 = x;
        self.cursor.1 = y;
    }

    fn focus(&mut self) {
        if self.buffer.cursor.0 < self.cursor.0 {
            self.cursor.0 = self.buffer.cursor.0.clone() - 1;
        } else if self.buffer.cursor.0 - self.cursor.0.clone() >= BUFFER_HEIGHT {
            self.cursor.0 = self.buffer.cursor.0.clone() - BUFFER_HEIGHT + 1;
        } else if self.buffer.cursor.1 < self.cursor.1 {
            self.cursor.1 = self.buffer.cursor.1.clone() - 1;
        } else if self.buffer.cursor.1 - self.cursor.1.clone() >= BUFFER_WIDTH {
            self.cursor.1 = self.buffer.cursor.1.clone() - BUFFER_WIDTH + 1;
        }

        let color_code = self.color_code;

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                if let Some(character) = self.buffer.char_at((row + self.cursor.0, col + self.cursor.1)) {
                    self.buffer_window.chars[row][col].write(ScreenChar {
                        ascii_character: character as u8,
                        color_code,
                    });
                } else {
                    self.buffer_window.chars[row][col].write(ScreenChar {
                        ascii_character: b' ',
                        color_code,
                    });
                }
            }
        }

        self.terminal_set_cursor();
    }

    pub fn clear(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer.clear();

        for row in 0..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH {
                self.buffer_window.chars[row][col].write(blank);
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer_window.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer_window.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
