#![no_std]
#![no_main]

use vga_buffer::_print_some_test_string;

mod panic;
mod vga_buffer;
mod macros;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    _print_some_test_string();

    loop {}
}
