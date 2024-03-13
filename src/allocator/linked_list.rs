use core::ptr::NonNull;

use alloc::alloc::GlobalAlloc;

use crate::allocator::align_up;

use super::Locked;

pub struct LinkedListAllocator {
    head: ListRegion,
}

struct ListRegion {
    /// The size of the this node
    size: usize,
    next: Option<NonNull<ListRegion>>,
}

impl ListRegion {
    fn new(size: usize) -> Self {
        ListRegion { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const _ as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: ListRegion {
                size: 0,
                next: None,
            },
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.add_free_region(heap_start, heap_size);
        };
    }

    fn size_align(layout: core::alloc::Layout) -> (usize, usize) {
        let layout = layout
            .align_to(core::mem::align_of::<ListRegion>())
            .expect("adjust alignment failed")
            .pad_to_align();

        (
            layout.size().max(core::mem::size_of::<ListRegion>()),
            layout.align(),
        )
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, core::mem::align_of::<ListRegion>()), addr);
        assert!(size > core::mem::size_of::<ListRegion>());

        let mut region = ListRegion::new(size);
        region.next = self.head.next.take();
        let node_ptr = addr as *mut ListRegion;

        // ⬇️ 如果用普通方法， node 在离开作用域就被释放了，必须用 unsafe 的原始指针写入
        unsafe {
            node_ptr.write(region);
        };

        self.head.next = Some(unsafe { NonNull::new_unchecked(node_ptr) });
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(NonNull<ListRegion>, usize)> {
        let mut current = &mut self.head;

        while let Some(region) = current.next.as_mut() {
            if let Ok(start_addr) = Self::alloc_from_region(unsafe { region.as_ref() }, size, align)
            {
                let next_free_region = (unsafe { region.as_mut() }).next.take();
                let res = (current.next.take().unwrap(), start_addr);
                current.next = next_free_region;

                return Some(res);
            } else {
                current = unsafe { &mut (*region.as_ptr()) };
            }
        }

        None
    }

    fn alloc_from_region(region: &ListRegion, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        if (region.end_addr() - alloc_end) < core::mem::size_of::<ListRegion>() {
            return Err(());
        }

        Ok(alloc_start)
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("ptr overflow");
            let free_region_size = unsafe { (*region.as_ptr()).end_addr() } - alloc_end;

            if free_region_size > core::mem::size_of::<ListRegion>() {
                allocator.add_free_region(alloc_end, free_region_size);
            }

            alloc_start as *mut _
        } else {
            core::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);
        self.lock().add_free_region(ptr as usize, size);
    }
}
