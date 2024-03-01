#![no_std]
// lib.rs 在测试的时候有一个隐式的 main 函数生成
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// lib.rs 和 main.rs 会被编译器当作两个不同的 crate。
// cargo test 执行时， lib.rs 和 main.rs 也会分别去跑测试

pub mod interrupts;
pub mod macros;
pub mod mem;
pub mod serial;
pub mod tests;
pub mod vga_buffer;

use bootloader::{entry_point, BootInfo};
pub use tests::*;

use core::panic::PanicInfo;

// 初始化操作
pub fn init() {
    interrupts::gdt::init();

    // 初始化中断描述符表
    interrupts::idt::IDT.load();

    // 初始化可变成中断控制器（PIC）
    interrupts::pic::init();
}

pub fn hit_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
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
    hit_loop();
}

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    use bootloader::BootInfo;

    init();
    test_main();
    hit_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
