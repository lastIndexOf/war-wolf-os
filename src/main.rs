#![no_std]
#![no_main]

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
