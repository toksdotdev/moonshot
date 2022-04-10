#![no_std]
#![no_main]

mod display;
use self::display::vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Dispay up!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);
    loop {}
}
