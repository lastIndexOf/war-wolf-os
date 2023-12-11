#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wolf_os::test_runner)]
#![reexport_test_harness_main = "_test_main"]

use core::{panic::PanicInfo, ptr::read_volatile};

use wolf_os::{
    println,
    vga_buffer::{ScreenChar, BUFFER_HEIGHT, WRITER},
};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    _test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wolf_os::test_panic_handler(info)
}

#[test_case]
fn test_correct_output_in_stdout_at_basic_boot() {
    let output = "stdout should show this line";
    println!("{output}");

    for (i, c) in output.chars().enumerate() {
        let screen_char: ScreenChar =
            unsafe { read_volatile(&WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i] as *const _) };

        assert_eq!(char::from(screen_char.ascii_code), c);
    }
}
