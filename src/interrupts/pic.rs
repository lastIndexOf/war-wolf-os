/// programming interrupts controller
/// CPU exception has 32 entries(0-32)
/// PIC has 15 entries(0-14), conflict with CPU
/// need remap PIC to 32-47
use pic8259::ChainedPics;
use spin::Mutex;

pub const MAIN_PIC_OFFSET: u8 = 32;
pub const SUB_PIC_OFFSET: u8 = 32 + 8;

pub static PIC: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(MAIN_PIC_OFFSET, SUB_PIC_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HardwareInterruptIndex {
    Timer = MAIN_PIC_OFFSET,
    Keyboard,
}

impl HardwareInterruptIndex {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn as_usize(&self) -> usize {
        self.as_u8() as usize
    }
}

pub fn init() {
    unsafe {
        PIC.lock().initialize();
    };

    x86_64::instructions::interrupts::enable();
}
