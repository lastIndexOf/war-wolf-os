#![no_std]
#![no_main]

mod macros;
mod panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    loop {}
}
