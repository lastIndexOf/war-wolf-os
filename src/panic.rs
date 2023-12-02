use core::panic::PanicInfo;

use crate::println;

// Divergent function, never has a return value.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
