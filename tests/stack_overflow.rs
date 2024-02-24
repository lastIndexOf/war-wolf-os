#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(type_name_of_val)]
#![feature(custom_test_frameworks)]
// 对于只跑单个测试的情况，不需要额外定义 test_runner，直接输出在 _start 中运行唯一的那个测试就行
// #![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]

use core::{panic::PanicInfo, ptr::read_volatile};

use lazy_static::lazy_static;
use wolf_os::{
    _exit_qemu, hit_loop,
    interrupts::{self, tss::DEFAULT_DOUBLE_FAULT_STACK_INDEX},
    serial_print, serial_println,
    vga_buffer::{ScreenChar, BUFFER_HEIGHT, WRITER},
    QemuExitCode, Testable,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(DEFAULT_DOUBLE_FAULT_STACK_INDEX as u16);
        }

        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    _exit_qemu(QemuExitCode::Success);
    hit_loop();
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_print!("running stack_overflow::stack_overflow...\t");

    interrupts::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

fn stack_overflow() -> i32 {
    let mut tmp = 0;

    stack_overflow();

    tmp += 1;
    tmp
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wolf_os::test_panic_handler(info)
}
