use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr,
};

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/// bootloader 启动时会把自己载入虚拟内存地址 0 - 1Mib 的区域
/// 所以这个区域默认已经完成了虚拟地址到物理地址的映射，即页表已经创建
/// 所以用这个区域来映射时是不需要分配页表，即 allocator 不会起作用
/// 但是如果分配其他区域，可能那个区域还没有与物理地址发生映射，也就没有低级页表
/// 可能就需要分配器创建页表
pub unsafe fn create_page_mapping_to_vga_buffer_example(
    page: Page,
    offset_page_table: &mut OffsetPageTable,
    allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    // FIXME: no safe, remove bottom
    let map_to_res =
        unsafe { offset_page_table.map_to(page, frame, flags, allocator) }.expect("map_to failed");
    map_to_res.flush();
}
