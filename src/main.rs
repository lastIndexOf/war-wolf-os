#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "_test_main"]
#![no_std]
#![no_main]

mod panic;

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use wolf_os::{
    hit_loop,
    mem::{mapping::MappingFrameAllocator, offset_page_mapper::init_offset_page_table},
    multitasking::co::{
        executor::base::BaseExecutor,
        task::{keyboard::print_keycode, Task},
    },
};

#[cfg(not(test))]
use wolf_os::println;

#[cfg(test)]
use wolf_os::{
    println, serial_println,
    tests::{QemuExitCode, Testable, _exit_qemu},
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

async fn async_number() {
    println!("print async number: 69");
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    wolf_os::init();
    println!("System initialized!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_offset_page_table(phys_mem_offset) };
    let mut allocator = unsafe { MappingFrameAllocator::new(&boot_info.memory_map) };
    wolf_os::mem::heap::init_heap(&mut mapper, &mut allocator).expect("heap initialization failed");

    let mut executor = BaseExecutor::new();
    executor.spawn(Task::new(print_keycode()));
    executor.spawn(Task::new(async_number()));
    executor.run();

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
