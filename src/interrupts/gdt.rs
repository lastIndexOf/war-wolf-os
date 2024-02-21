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

pub fn update_segment_registers() {
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}
