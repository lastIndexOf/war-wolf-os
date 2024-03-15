use spin::Mutex;

use self::{
    bump::BumpAllocator, fixed_size_block::FixedSizeAllocator, linked_list::LinkedListAllocator,
};

mod bump;
mod dummy;
mod fixed_size_block;
mod linked_list;

// #[global_allocator]
// pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
// #[global_allocator]
// pub static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
#[global_allocator]
pub static ALLOCATOR: Locked<FixedSizeAllocator> = Locked::new(FixedSizeAllocator::new());

pub struct Locked<T> {
    inner: Mutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

pub fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;

    if remainder == 0 {
        addr
    } else {
        addr - remainder + align
    }
}
