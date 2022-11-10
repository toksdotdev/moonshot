use super::PIC_1_OFFSET;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    /// Interrupt arrives at line 32 (0 + 32 offset).
    /// Assigning the timer to the `PIC_1_OFFSET` means
    /// subsequent variants of the InterruptIndex enum will
    /// have its value incremented from the previous value's
    /// offset.
    Timer = PIC_1_OFFSET,

    /// Interrupt arrives at line 33 (1 + 32 offset of timer).
    Keyboard,
}

impl InterruptIndex {
    pub(crate) fn as_u8(self) -> u8 {
        self as u8
    }

    pub(crate) fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
