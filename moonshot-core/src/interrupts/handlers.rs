use moonshot_display::print;
use moonshot_display::println;
use pc_keyboard::DecodedKey;
use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupts::index::InterruptIndex;
use crate::interrupts::PICS;

use lazy_static::lazy_static;
use pc_keyboard::layouts;
use pc_keyboard::HandleControl;
use pc_keyboard::Keyboard;
use pc_keyboard::ScancodeSet1;
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(HandleControl::Ignore));
}

pub(crate) extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("exception: breakpoint exception\n{:#?}", stack_frame);
}

pub(crate) extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("exception: double fault\n{:#?}", stack_frame);
}

pub(crate) extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub(crate) extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode = unsafe { port.read() };
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(event) {
            match key {
                DecodedKey::RawKey(key) => print!("{:?}", key),
                DecodedKey::Unicode(character) => print!("{:?}", character),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
