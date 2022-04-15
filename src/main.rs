#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod display;
mod exit;
mod serial;
#[cfg(test)]
mod testing;

use self::display::vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Dispay up!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    loop {}
}
