#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod display;
pub mod exit;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod testing;

pub fn initialize() {
    gdt::initialize();
    interrupts::initialize_idt();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    initialize();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::testing::panic_handler(info);
}
