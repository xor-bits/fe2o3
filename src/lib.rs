#![no_std]
#![no_main]
#![feature(try_blocks)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_fw::test_runner)]
#![reexport_test_harness_main = "test_main"]

//

use raw_cpuid::CpuId;

pub static KERNEL_NAME: &str = if cfg!(test) { "Fe2O3-Testing" } else { "Fe2O3" };

//

pub mod int;
pub mod log;
pub mod panic;
pub mod serial;
pub mod test_fw;
pub mod vga;

//

#[no_mangle]
#[link_section = ".text"]
pub fn kernel_main() {
    init();
}

pub fn init() {
    // clear the screen
    print!();

    // print a friendly hello message
    hello();

    // interrupt descriptor table
    int::init_idt();

    let cpuid = CpuId::new();

    if let Some(vendor) = cpuid.get_vendor_info() {
        debug!("CPU: {}", vendor.as_str());
    }
    if let Some(cparams) = cpuid.get_feature_info() {
        debug!("Has APIC: {}", cparams.has_apic());
    }

    // debug breakpoint exception
    debug!("Debug breakpoint coming up");
    x86_64::instructions::interrupts::int3();
    debug!("Debug breakpoint worked");
}

pub fn hello() {
    // boot up message
    info!("Hello from the {KERNEL_NAME} kernel!");
    serial_println!("Hello from the {KERNEL_NAME} kernel!");
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
