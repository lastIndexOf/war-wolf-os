use spin::Mutex;

use self::{bump::BumpAllocator, linked_list::LinkedListAllocator};

pub mod bump;
pub mod dummy;
pub mod linked_list;

// #[global_allocator]
// pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
#[global_allocator]
pub static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

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
