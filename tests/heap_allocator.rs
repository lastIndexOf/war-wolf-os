#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wolf_os::test_runner)]
#![reexport_test_harness_main = "_test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use wolf_os::{
    hit_loop,
    mem::{
        heap::HEAP_SIZE, mapping::MappingFrameAllocator, offset_page_mapper::init_offset_page_table,
    },
};
use x86_64::VirtAddr;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    wolf_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_offset_page_table(phys_mem_offset) };
    let mut allocator = unsafe { MappingFrameAllocator::new(&boot_info.memory_map) };
    wolf_os::mem::heap::init_heap(&mut mapper, &mut allocator).expect("heap initialization failed");

    _test_main();
    hit_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wolf_os::test_panic_handler(info)
}

#[test_case]
fn test_box_alloc_in_heap() {
    let heap_val = Box::new(41);
    assert_eq!(*heap_val, 41);
}

#[test_case]
fn test_large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn test_heap_auto_dealloc_and_reuse() {
    for i in 0..(HEAP_SIZE + 100) {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
