use core::ptr::NonNull;

use alloc::alloc::GlobalAlloc;

use crate::allocator::align_up;

pub struct LinkedListAllocator {
    head: ListNode,
}

struct ListNode {
    /// The size of the this node
    size: usize,
    next: Option<NonNull<ListNode>>,
}

impl ListNode {
    fn new(size: usize) -> Self {
        ListNode { size, next: None }
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
            head: ListNode {
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

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, core::mem::align_of::<ListNode>()), addr);
        assert!(size > core::mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;

        // ⬇️ 如果用普通方法， node 在离开作用域就被释放了，必须用 unsafe 的原始指针写入
        unsafe {
            node_ptr.write(node);
        };

        self.head.next = Some(unsafe { NonNull::new_unchecked(node_ptr) });
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(NonNull<ListNode>, usize)> {
        let mut current = &mut self.head;

        while let Some(node) = current.next.as_mut() {
            if let Ok(start_addr) = Self::alloc_from_region(unsafe { node.as_ref() }, size, align) {
                let next_free_node = (unsafe { node.as_mut() }).next.take();
                let res = (current.next.take().unwrap(), start_addr);
                current.next = next_free_node;

                return Some(res);
            } else {
                current = unsafe { &mut (*node.as_ptr()) };
            }
        }

        None
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        todo!()
    }
}

unsafe impl GlobalAlloc for LinkedListAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
