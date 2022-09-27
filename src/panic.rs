use core::panic::PanicInfo;

//

#[cfg(not(feature = "tests"))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    crate::error!("{}", info);
    crate::halt()
}

#[cfg(feature = "tests")]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    crate::test_fw::test_panic_handler(info)
}
