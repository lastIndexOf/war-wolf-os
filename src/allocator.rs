use self::dummy::DummyAllocator;

pub mod dummy;
pub mod heap;

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;
