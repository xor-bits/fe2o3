use crate::vga::{Color, ColorCode, WRITER};
use core::{
    fmt::{Arguments, Write},
    sync::atomic::{AtomicU16, Ordering},
};

//

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::_log(format_args!($($arg)*), $crate::log::LogLevel::Trace)
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::_log(format_args!($($arg)*), $crate::log::LogLevel::Debug)
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::_log(format_args!($($arg)*), $crate::log::LogLevel::Info)
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::_log(format_args!($($arg)*), $crate::log::LogLevel::Warn)
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::_log(format_args!($($arg)*), $crate::log::LogLevel::Error)
    }
}

//

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

//

impl LogLevel {
    pub fn color_code(self) -> ColorCode {
        match self {
            LogLevel::Trace => ColorCode::new(Color::LightCyan, Color::Black),
            LogLevel::Debug => ColorCode::new(Color::Green, Color::Black),
            LogLevel::Info => ColorCode::new(Color::Blue, Color::Black),
            LogLevel::Warn => ColorCode::new(Color::Yellow, Color::Black),
            LogLevel::Error => ColorCode::new(Color::Red, Color::Black),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info ",
            LogLevel::Warn => "warn ",
            LogLevel::Error => "error",
        }
    }
}

//

pub fn _log(args: Arguments, level: LogLevel) {
    let mut writer = WRITER.lock();

    static TIMESTAMP_COUNTER: AtomicU16 = AtomicU16::new(0);

    // timestamp
    writer.set_color_code(ColorCode::new(Color::LightGrey, Color::Black));
    writer.write_str("[");
    writer.set_color_code(ColorCode::new(Color::White, Color::Black));
    writer
        .write_fmt(format_args!(
            "{:06}",
            TIMESTAMP_COUNTER.fetch_add(1, Ordering::SeqCst)
        ))
        .unwrap(); // TODO: Actual timestamp
    writer.set_color_code(ColorCode::new(Color::LightGrey, Color::Black));
    writer.write_str("] [");

    // log level
    writer.set_color_code(level.color_code());
    writer.write_str(level.name()); // TODO: Actual timestamp
    writer.set_color_code(ColorCode::new(Color::LightGrey, Color::Black));
    writer.write_str("] ");

    // log msg
    writer.set_color_code(ColorCode::new(Color::White, Color::Black));
    writer.write_fmt(args).unwrap();
    writer.write_byte(b'\n');
}
