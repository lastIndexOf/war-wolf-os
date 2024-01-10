#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod panic;

#[cfg(not(test))]
use wolf_os::println;

#[cfg(test)]
use wolf_os::{
    println, serial_println,
    tests::{QemuExitCode, Testable, _exit_qemu},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    wolf_os::init();

    println!("System initialized!");

    // 访问无效内存地址，触发 page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 12;
    // }

    // 栈溢出，调用栈返回到栈底的 guard page
    // 这个 guard page 实际上没有关联物理内存，触发 page fault
    // fn stack_overflow() {
    //     stack_overflow(); // 每一次递归都会将返回地址入栈
    // }

    // // 触发 stack overflow
    // stack_overflow();

    // cargo test 会生成一个默认的启动函数 main。
    // 在 no_main 环境下不会自动调用，因此需要主动调用
    #[cfg(test)]
    _test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    _exit_qemu(QemuExitCode::Success);
}
