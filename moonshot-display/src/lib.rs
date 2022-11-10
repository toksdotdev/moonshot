#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(moonshot_shared::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(feature = "vga")]
pub mod vga;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    moonshot_shared::testing::panic_handler(info)
}
