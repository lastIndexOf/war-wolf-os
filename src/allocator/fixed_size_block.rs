use core::ptr::NonNull;

use alloc::alloc::GlobalAlloc;

use super::Locked;

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

fn find_list_index_by_layout(layout: core::alloc::Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES
        .iter()
        .position(|&ele| ele >= required_block_size)
}

pub struct FixedSizeAllocator {
    heads: [Option<NonNull<ListRegion>>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

#[derive(Debug)]
struct ListRegion {
    next: Option<NonNull<ListRegion>>,
}

unsafe impl Send for ListRegion {}
unsafe impl Send for FixedSizeAllocator {}

impl FixedSizeAllocator {
    pub const fn new() -> Self {
        FixedSizeAllocator {
            heads: [None; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.fallback_allocator
                .init(heap_start as *mut _, heap_size);
        };
    }

    fn fallback_alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ret) => ret.as_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match find_list_index_by_layout(layout) {
            Some(idx) => match allocator.heads[idx].take() {
                Some(region) => {
                    allocator.heads[idx] = unsafe { (*region.as_ptr()).next.take() };
                    region.as_ptr() as *mut u8
                }
                None => {
                    let block_size = BLOCK_SIZES[idx];
                    let block_align = block_size;
                    let layout = unsafe {
                        core::alloc::Layout::from_size_align_unchecked(block_size, block_align)
                    };
                    allocator.fallback_alloc(layout)
                }
            },
            None => return allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.lock();

        match find_list_index_by_layout(layout) {
            Some(idx) => {
                let new_region = ListRegion {
                    next: allocator.heads[idx].take(),
                };

                assert!(core::mem::size_of::<ListRegion>() <= BLOCK_SIZES[idx]);
                assert!(core::mem::align_of::<ListRegion>() <= BLOCK_SIZES[idx]);

                let new_region_ptr = ptr as *mut ListRegion;
                unsafe {
                    new_region_ptr.write(new_region);
                };

                allocator.heads[idx] = Some(unsafe { NonNull::new_unchecked(new_region_ptr) });
            }
            None => unsafe {
                allocator
                    .fallback_allocator
                    .deallocate(NonNull::new_unchecked(ptr), layout);
            },
        }
    }
}
