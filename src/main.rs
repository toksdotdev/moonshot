#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(moonshot::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use moonshot::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Dispay up!");
    moonshot::initialize();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use moonshot::eprintln;

    eprintln!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    moonshot::testing::panic_handler(info);
}
