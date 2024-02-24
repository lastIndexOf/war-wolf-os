#![no_std]
#![no_main]
#![feature(type_name_of_val)]
#![feature(custom_test_frameworks)]
// 对于只跑单个测试的情况，不需要额外定义 test_runner，直接输出在 _start 中运行唯一的那个测试就行
// #![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]

use core::{panic::PanicInfo, ptr::read_volatile};

use wolf_os::{
    _exit_qemu, hit_loop, serial_print, serial_println,
    vga_buffer::{ScreenChar, BUFFER_HEIGHT, WRITER},
    QemuExitCode, Testable,
};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // 这个集成测试是测试的需要 panic 的一些功能，因此 test() 运行不能正常闭合，需要出发 panic_handler
    should_panic();
    serial_println!("[test did not panic]");
    _exit_qemu(QemuExitCode::Failed);

    hit_loop();
}

fn should_panic() {
    serial_print!("running {}... ", core::any::type_name_of_val(&should_panic));

    assert_eq!(1, 0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    // 这种方法的缺点是它只使用于单个的测试函数。
    // 对于多个 #[test_case] 函数，它只会执行第一个函数，因为程序无法在panic处理被调用后继续执行。
    _exit_qemu(QemuExitCode::Success);

    hit_loop();
}
