#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

//

use int::init_idt;

#[cfg(feature = "tests")]
mod test_fw;

mod int;
mod log;
mod panic;
mod serial;
mod vga;

//

#[no_mangle]
#[link_section = ".text"]
pub fn kernel_main() {
    init();

    #[cfg(feature = "tests")]
    {
        test_fw::test_runner(&[]);
    }

    // run app
    /* for i in 0.. {
        println!("Init {i}");
    } */
}

pub fn init() {
    // clear the screen
    print!();

    // print a friendly hello message
    hello();

    // interrupt descriptor table
    init_idt();

    // debug breakpoint exception
    println!("Debug breakpoint coming up");
    x86_64::instructions::interrupts::int3();
    println!("Debug breakpoint worked");
}

pub fn hello() {
    // boot up message
    #[cfg(feature = "tests")]
    serial_println!("Hello from the Fe2O3 Testing kernel!");
    #[cfg(not(feature = "tests"))]
    println!("Hello from the Fe2O3 kernel!");
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
