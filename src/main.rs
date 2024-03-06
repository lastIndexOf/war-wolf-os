#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod panic;

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use wolf_os::hit_loop;

#[cfg(not(test))]
use wolf_os::println;

#[cfg(test)]
use wolf_os::{
    println, serial_println,
    tests::{QemuExitCode, Testable, _exit_qemu},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    wolf_os::init();
    println!("System initialized!");

    let b = Box::new(1);

    // cargo test 会生成一个默认的启动函数 main。
    // 在 no_main 环境下不会自动调用，因此需要主动调用
    #[cfg(test)]
    _test_main();

    hit_loop();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    _exit_qemu(QemuExitCode::Success);
}
