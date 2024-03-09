use alloc::alloc::GlobalAlloc;

use super::{align_up, Locked};

pub struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_size: 0,
            next: 0,
            allocations: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_size = heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut inner = self.lock();

        let alloc_size = layout.size();
        let alloc_start = align_up(inner.next, layout.align());

        let alloc_end = match alloc_start.checked_add(alloc_size) {
            Some(end) => end,
            None => return core::ptr::null_mut(),
        };

        if alloc_end > inner.heap_start + inner.heap_size {
            return core::ptr::null_mut();
        }

        inner.next = alloc_end;
        inner.allocations += 1;

        alloc_start as *mut _
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        let mut inner = self.lock();

        inner.allocations -= 1;
        if inner.allocations == 0 {
            inner.next = inner.heap_start;
        }
    }
}
