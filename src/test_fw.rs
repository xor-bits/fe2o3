use crate::{halt, println, serial_print, serial_println};
use core::{any::type_name, panic::PanicInfo};
use x86_64::instructions::port::Port;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub trait TestCase {
    fn run(&self);
}

//

impl<F: Fn()> TestCase for F {
    fn run(&self) {
        serial_print!("running {} ...\t", type_name::<Self>());
        self();
        serial_println!("[ok]");
    }
}

//

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn TestCase]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("{}\n", info);

    println!("{}\n\n:(", info);
    exit_qemu(QemuExitCode::Failed);
    halt();
}

//
