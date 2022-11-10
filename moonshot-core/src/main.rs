#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(moonshot_shared::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use moonshot_display::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Dispay up!");
    moonshot_core::initialize();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    moonshot_display::eprintln!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    moonshot_shared::testing::panic_handler(info);
}
