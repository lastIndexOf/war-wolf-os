use lazy_static::lazy_static;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DEFAULT_DOUBLE_FAULT_STACK_INDEX: usize = 0;

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DEFAULT_DOUBLE_FAULT_STACK_INDEX] = {
            const STACK_SIZE: usize = 4096 * 5;
            // 用 static mut 的数组模拟一下中断栈表
            // TODO: 实现了内容页后改掉
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK as *const _ });
            let stack_end = stack_start + STACK_SIZE;

            stack_end
        };

        tss
    };
}
