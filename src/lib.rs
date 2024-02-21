#![no_std]
// lib.rs 在测试的时候有一个隐式的 main 函数生成
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// lib.rs 和 main.rs 会被编译器当作两个不同的 crate。
// cargo test 执行时， lib.rs 和 main.rs 也会分别去跑测试

pub mod interrupts;
pub mod macros;
pub mod serial;
pub mod tests;
pub mod vga_buffer;

pub use tests::*;

use core::panic::PanicInfo;

// 初始化操作
pub fn init() {
    // 初始化全局描述符表
    interrupts::gdt::GDT.0.load();
    // 载入 gdt 后，手动修改全局段寄存器的状态，因为载入 gdt 不会更新那些段寄存器的状态，需要手动更新
    // 通过切换中断栈帧，解决 stack overflow 后会导致 double fault 也无法正常入栈的问题
    interrupts::gdt::update_segment_registers();

    // 初始化中断描述符表
    interrupts::idt::IDT.load();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    _exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    _exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
