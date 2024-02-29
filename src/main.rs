#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod panic;

use bootloader::{entry_point, BootInfo};
#[cfg(not(test))]
use wolf_os::println;
use wolf_os::{hit_loop, mem::page::get_l4_table};

#[cfg(test)]
use wolf_os::{
    println, serial_println,
    tests::{QemuExitCode, Testable, _exit_qemu},
};
use x86_64::structures::paging::PageTable;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    wolf_os::init();

    println!("System initialized!");

    let physical_memory_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { get_l4_table(physical_memory_offset) };

    for (idx, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            // l4 page item data
            println!("L4 Entry {}: {:?}", idx, entry);

            let l3_addr = physical_memory_offset + entry.frame().unwrap().start_address().as_u64();
            let l3_ptr: *const PageTable = l3_addr.as_ptr();
            let l3_table = unsafe { &*l3_ptr };

            for (idx, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    // l3 page item data
                    println!("L3 Entry {}: {:?}", idx, entry);
                }
            }
        }
    }

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
