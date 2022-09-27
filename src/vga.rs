use core::{
    fmt::{Arguments, Write},
    ops::{Deref, DerefMut},
};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

//

#[macro_export]
macro_rules! println {
    () => {
        println!("");
    };

    ($($arg:tt)*) => {
        $crate::vga::_println(format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! print {
    () => {
        print!("");
    };

    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*))
    };
}

pub fn _print(args: Arguments) {
    let mut writer = WRITER.lock();
    writer.write_fmt(args).unwrap();
}

pub fn _println(args: Arguments) {
    let mut writer = WRITER.lock();
    writer.write_fmt(args).unwrap();
    writer.write_byte(b'\n');
}

// SAFETY: `Writer::init` is private and this is the only caller
//

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::init());
}

//

pub struct Writer {
    row: usize,
    column: usize,
    color_code: ColorCode,
}

impl Writer {
    pub fn write_str(&mut self, s: &str) {
        for byte in s
            .bytes()
            .filter(|b| (b' '..=b'~').contains(b) || *b == b'\n' || *b == b'\r')
        {
            self.write_byte(byte)
        }
    }

    pub fn write_char(&mut self, c: char) {
        self.write_str(c.encode_utf8(&mut [0; 4]));
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\r' => self.column = 0,
            b'\n' => self.new_line(),
            code_point => {
                if self.column >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row;
                let col = self.column;

                let color = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar { code_point, color });
                self.column += 1;
            }
        }
    }

    pub fn set_color_code(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn init() -> Self {
        let mut result = Self {
            row: 0,
            column: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
        };
        result.clear();
        result
    }

    fn buffer(&mut self) -> &'static mut Buffer {
        // SAFETY: only one `Writer` should exist at one time
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    }

    fn new_line(&mut self) {
        if self.row + 1 == VGA_BUFFER_HEIGHT {
            for row in 0..VGA_BUFFER_HEIGHT - 1 {
                for col in 0..VGA_BUFFER_WIDTH {
                    let tmp = self.buffer().chars[row + 1][col].read();
                    self.buffer().chars[row][col].write(tmp);
                }
            }
        } else {
            self.row += 1;
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    fn clear(&mut self) {
        for row in 0..VGA_BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn clear_row(&mut self, row: usize) {
        self.fill_row(
            row,
            ScreenChar {
                code_point: b' ',
                color: ColorCode::default(),
            },
        )
    }

    fn fill_row(&mut self, row: usize, fill: ScreenChar) {
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer().chars[row][col].write(fill);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

//

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub enum Color {
    #[default]
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    // ascii
    code_point: u8,

    // bg & fg
    color: ColorCode,
}

impl Deref for ScreenChar {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

//

const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}
