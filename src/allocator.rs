use self::dummy::DummyAllocator;

pub mod dummy;

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;
