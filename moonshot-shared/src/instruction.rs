/// Halt the CPU until an external interrupt is triggered.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
