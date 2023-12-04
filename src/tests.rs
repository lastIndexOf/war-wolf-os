#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("running {} tests", tests.len());

    for test in tests {
        test();
    }

    _exit_qemu(QemuExitCode::Success);
}

fn _exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // isa-debug-exit Exit设备的端口
    let mut port = Port::new(0xf4);
    unsafe {
        port.write(exit_code as u32);
    };
}
