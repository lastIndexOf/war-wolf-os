//! x86_64 use Level 4 page table
//! 4kb per page size
//! 1page (4kb) per page table
//! 512 items per page table (4kb / item)

use x86_64::{registers::control::Cr3, structures::paging::PageTable, PhysAddr, VirtAddr};

pub unsafe fn get_l4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (l4_table_frame, _) = Cr3::read();
    let l4_table_phys_addr = l4_table_frame.start_address();

    let l4_table_vir_addr = physical_memory_offset + l4_table_phys_addr.as_u64();
    let l4_table_ptr: *mut PageTable = l4_table_vir_addr.as_mut_ptr();

    unsafe { &mut *l4_table_ptr }
}

pub unsafe fn transform_vir_addr_to_phys_addr(
    vir_addr: VirtAddr,
    physical_memory_offset: VirtAddr,
) -> Option<PhysAddr> {
    transform_vir_addr_to_phys_addr_safe(vir_addr, physical_memory_offset)
}

fn transform_vir_addr_to_phys_addr_safe(
    vir_addr: VirtAddr,
    physical_memory_offset: VirtAddr,
) -> Option<PhysAddr> {
    let page_table_indexes = [
        vir_addr.p4_index(),
        vir_addr.p3_index(),
        vir_addr.p2_index(),
        vir_addr.p1_index(),
    ];

    let mut frame = Cr3::read().0;

    for index in page_table_indexes {
        let cur_page_table_addr = physical_memory_offset + frame.start_address().as_u64();
        let cur_page_table_ptr = cur_page_table_addr.as_ptr();
        let cur_page_table: &PageTable = unsafe { &*cur_page_table_ptr };

        frame = match cur_page_table[index].frame() {
            Ok(frame) => frame,
            Err(_) => return None,
        }
    }

    Some(frame.start_address() + u64::from(vir_addr.page_offset()))
}

/// 让 bootloader 设置递归页表为 0o777 索引后
/// 就可以通过以下逻辑来实现虚拟地址到物理地址的映射
#[allow(unused)]
pub fn visit_physical_mem_from_virtual_addr(virtual_addr: u64) {
    // 八进制，4级页表的递归索引号
    let recursive_index_at_l4 = 0o777;
    let sign_extend = 0o177777 << 48;

    // 39 = 12 + 9 * 3
    let l4_idx = (virtual_addr >> 39) & 0o777;
    let l3_idx = (virtual_addr >> 30) & 0o777;
    let l2_idx = (virtual_addr >> 21) & 0o777;
    let l1_idx = (virtual_addr >> 12) & 0o777;

    let page_offset = virtual_addr & 0o7777;

    let l4_table_addr = sign_extend
        | (recursive_index_at_l4 << 39)
        | (recursive_index_at_l4 << 30)
        | (recursive_index_at_l4 << 21)
        | (recursive_index_at_l4 << 12);
    let l3_table_addr = sign_extend
        | (recursive_index_at_l4 << 39)
        | (recursive_index_at_l4 << 30)
        | (recursive_index_at_l4 << 21)
        | (l4_idx << 12);
    let l2_table_addr = sign_extend
        | (recursive_index_at_l4 << 39)
        | (recursive_index_at_l4 << 30)
        | (l4_idx << 21)
        | (l3_idx << 12);
    let l1_table_addr = sign_extend
        | (recursive_index_at_l4 << 39)
        | (l4_idx << 30)
        | (l3_idx << 21)
        | (l2_idx << 12);
}
