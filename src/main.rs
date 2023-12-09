#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod macros;
mod panic;
mod serial;
mod tests;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    // cargo test 会生成一个默认的启动函数 main。
    // 在 no_main 环境下不会自动调用，因此需要主动调用
    #[cfg(test)]
    _test_main();

    loop {}
}

#[test_case]
fn first_test_case() {
    assert_eq!(1, 1);
}

#[test_case]
fn error_test_case() {
    assert_eq!(1, 1);
}
