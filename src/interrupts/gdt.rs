use lazy_static::lazy_static;
use x86_64::{
    registers::segmentation::Segment,
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

use crate::interrupts::tss::TSS;

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    // 初始化全局描述符表
    GDT.0.load();
    // 载入 gdt 后，手动修改全局段寄存器的状态，因为载入 gdt 不会更新那些段寄存器的状态，需要手动更新
    // 通过切换中断栈帧，解决 stack overflow 后会导致 double fault 也无法正常入栈的问题
    update_segment_registers();
}

fn update_segment_registers() {
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}
