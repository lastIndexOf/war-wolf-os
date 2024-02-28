//! x86_64 use Level 4 page table
//! 4kb per page size
//! 1page (4kb) per page table
//! 512 items per page table (4kb / item)

/// 让 bootloader 设置递归页表为 0o777 索引后
/// 就可以通过以下逻辑来实现虚拟地址到物理地址的映射
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
