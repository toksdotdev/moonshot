#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(moonshot_shared::testing::test_runner)]

use core::panic::PanicInfo;
use moonshot_display::println;
use moonshot_shared::testing;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    moonshot_shared::instruction::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("Boot successful without panic");
}
