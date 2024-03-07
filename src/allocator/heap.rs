use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

/// 100 KiB
pub const HEAP_SIZE: usize = 100 * 1024;
pub const HEAP_START: usize = 0x_4444_4444_0000;

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let pages = {
        let heap_start_addr = VirtAddr::new(HEAP_START as u64);
        let heap_end_addr = heap_start_addr + HEAP_SIZE - 1u64;
        let heap_start_page = Page::<Size4KiB>::containing_address(heap_start_addr);
        let heap_end_page = Page::<Size4KiB>::containing_address(heap_end_addr);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in pages {
        let phy_frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper
                .map_to(page, phy_frame, flags, frame_allocator)?
                .flush();
        };
    }

    Ok(())
}
