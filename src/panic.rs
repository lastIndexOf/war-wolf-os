use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use wolf_os::println;

    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use wolf_os::{
        serial_println,
        tests::{QemuExitCode, _exit_qemu},
    };

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    _exit_qemu(QemuExitCode::Failed);
    loop {}
}
