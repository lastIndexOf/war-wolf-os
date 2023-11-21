use core::panic::PanicInfo;

// Divergent function, never has a return value.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
