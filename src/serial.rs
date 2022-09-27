use crate::error;
use core::fmt::{Arguments, Error, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

//

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
        $crate::serial::_println(format_args!($($arg)*));
    };
}

pub fn _print(args: Arguments) {
    let mut writer = SERIAL1.lock();
    if let Err::<(), Error>(_) = try {
        writer.write_fmt(args)?;
        writer.write_char('\n')?;
    } {
        error!("Failed to write to serial");
    }
}

pub fn _println(args: Arguments) {
    let mut writer = SERIAL1.lock();
    if let Err::<(), Error>(_) = try {
        writer.write_fmt(args)?;
        writer.write_char('\n')?;
    } {
        error!("Failed to write to serial");
    }
}
