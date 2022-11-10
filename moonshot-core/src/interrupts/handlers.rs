use moonshot_display::print;
use moonshot_display::println;
use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupts::index::InterruptIndex;
use crate::interrupts::PICS;

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
