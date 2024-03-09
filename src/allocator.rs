use spin::Mutex;

use self::bump::BumpAllocator;

pub mod bump;
pub mod dummy;

#[global_allocator]
pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

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
