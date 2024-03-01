#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod panic;

use core::ptr::write_volatile;

use bootloader::{entry_point, BootInfo};
#[cfg(not(test))]
use wolf_os::println;
use wolf_os::{
    hit_loop,
    mem::{
        mapping::{create_page_mapping_to_vga_buffer_example, MappingFrameAllocator},
        offset_page_mapper::init_offset_page_table,
    },
};

#[cfg(test)]
use wolf_os::{
    println, serial_println,
    tests::{QemuExitCode, Testable, _exit_qemu},
};
use x86_64::{structures::paging::Page, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    wolf_os::init();

    println!("System initialized!");

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
