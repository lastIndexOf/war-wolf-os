pub trait Testable {
    fn run(&self);
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        use crate::{serial_print, serial_println};

        serial_print!("running {}... ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn _exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // isa-debug-exit Exit设备的端口
    let mut port = Port::new(0xf4);
    unsafe {
        port.write(exit_code as u32);
    };
}
