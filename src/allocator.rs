use linked_list_allocator::LockedHeap;

pub mod dummy;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
