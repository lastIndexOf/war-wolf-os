use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
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

type UsableFrames = impl Iterator<Item = PhysFrame>;

pub struct MappingFrameAllocator {
    usable_frames: UsableFrames,
}

impl MappingFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        MappingFrameAllocator {
            usable_frames: MappingFrameAllocator::usable_frames_iter(memory_map),
        }
    }

    fn usable_frames_iter(memory_map: &'static MemoryMap) -> UsableFrames {
        memory_map
            .iter()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .map(|region| region.range.start_addr()..region.range.end_addr())
            .flat_map(|region_range| region_range.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for MappingFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.usable_frames.next()
    }
}

/// bootloader 启动时会把自己载入虚拟内存地址 0 - 1Mib 的区域
/// 所以这个区域默认已经完成了虚拟地址到物理地址的映射，即页表已经创建
/// 所以用这个区域来映射时是不需要分配页表，即 allocator 不会起作用
/// 但是如果分配其他区域，可能那个区域还没有与物理地址发生映射，也就没有低级页表
/// 可能就需要分配器创建页表
///
/// let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
/// let mut mapper = unsafe { init_offset_page_table(phys_mem_offset) };
///
/// let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
/// let mut allocator = unsafe { MappingFrameAllocator::new(&boot_info.memory_map) };
///
/// unsafe {
///     create_page_mapping_to_vga_buffer_example(page, &mut mapper, &mut allocator);
///     write_volatile(
///         page.start_address().as_mut_ptr::<u64>().offset(400),
///         0x_f021_f077_f065_f04e,
///     );
/// };
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
