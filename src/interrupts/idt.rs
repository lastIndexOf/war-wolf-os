/// Interrupt Descriptor Table
/// 中断描述符表
/// Type	Name	                    Description
/// u16	    Function Pointer [0:15]	    处理函数地址的低位（最后16位）
/// u16	    GDT selector	            全局描述符表中的代码段标记。
/// u16	    Options	（如下所述）
/// u16	    Function Pointer [16:31]	处理函数地址的中位（中间16位）
/// u32	    Function Pointer [32:63]	处理函数地址的高位（剩下的所有位）
/// u32	    Reserved
///
/// Options字段的格式如下：
/// Bits	Name	                           Description
/// 0-2	    Interrupt Stack Table Index	       0: 不要切换栈, 1-7: 当处理函数被调用时，切换到中断栈表的第n层。
/// 3-7	    Reserved
/// 8	    0: Interrupt Gate, 1: Trap Gate	   如果该比特被置为0，当处理函数被调用时，中断会被禁用。
/// 9-11	must be one
/// 12	    must be zero
/// 13‑14	Descriptor Privilege Level (DPL)   执行处理函数所需的最小特权等级。
/// 15	    Present
///
/// 当异常发生时，CPU会执行如下步骤：
///
/// 将一些寄存器数据入栈，包括指令指针以及 RFLAGS 寄存器。（我们会在文章稍后些的地方用到这些数据。）
/// 读取中断描述符表（IDT）的对应条目，比如当发生 page fault 异常时，调用14号条目。
/// 判断该条目确实存在，如果不存在，则触发 double fault 异常。
/// 如果该条目属于中断门（interrupt gate，bit 40 被设置为0），则禁用硬件中断。
/// 将 GDT 选择器载入代码段寄存器（CS segment）。
/// 跳转执行处理函数。
///
///
///
// 这个库里已经实现了 x86_64 的中断描述符表(IDT)，我们不需要单独再定义了

/// in x86_64;
/// pub struct InterruptDescriptorTable {
///     pub divide_error: Entry<HandlerFunc>,
///     pub debug: Entry<HandlerFunc>,
///     pub non_maskable_interrupt: Entry<HandlerFunc>,
///     pub breakpoint: Entry<HandlerFunc>,
///     pub overflow: Entry<HandlerFunc>,
///     pub bound_range_exceeded: Entry<HandlerFunc>,
///     pub invalid_opcode: Entry<HandlerFunc>,
///     pub device_not_available: Entry<HandlerFunc>,
///     pub double_fault: Entry<DivergingHandlerFuncWithErrCode>,
///     coprocessor_segment_overrun: Entry<HandlerFunc>,
///     pub invalid_tss: Entry<HandlerFuncWithErrCode>,
///     pub segment_not_present: Entry<HandlerFuncWithErrCode>,
///     pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
///     pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
///     pub page_fault: Entry<PageFaultHandlerFunc>,
///     reserved_1: Entry<HandlerFunc>,
///     pub x87_floating_point: Entry<HandlerFunc>,
///     pub alignment_check: Entry<HandlerFuncWithErrCode>,
///     pub machine_check: Entry<DivergingHandlerFunc>,
///     pub simd_floating_point: Entry<HandlerFunc>,
///     pub virtualization: Entry<HandlerFunc>,
///     pub cp_protection_exception: Entry<HandlerFuncWithErrCode>,
///     reserved_2: [Entry<HandlerFunc>; 6],
///     pub hv_injection_exception: Entry<HandlerFunc>,
///     pub vmm_communication_exception: Entry<HandlerFuncWithErrCode>,
///     pub security_exception: Entry<HandlerFuncWithErrCode>,
///     reserved_3: Entry<HandlerFunc>,
///     interrupts: [Entry<HandlerFunc>; 256 - 32],
/// }
// "x86-interrupt" 调用
// 它可以保证在函数返回时，寄存器里的值均返回原样。
// extern "x86-interrupt" fn();
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    hit_loop,
    interrupts::{
        hdw::KEYBOARD,
        pic::{HardwareInterruptIndex, PIC},
        tss::DEFAULT_DOUBLE_FAULT_STACK_INDEX,
    },
    print, println,
};

lazy_static! {
    /// 不注册处理函数，又触发了对应的错误时触发的是 general protection fault
    /// 如果不设置 double fault，x86_64 会在 double fault 时触发 triple fault，导致系统重启。
    /// breakpoint fault interrupt handler
    /// double fault interrupt handler
    /// real time clock hardware interrupt handler
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DEFAULT_DOUBLE_FAULT_STACK_INDEX as u16);
        };

        idt[HardwareInterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt[HardwareInterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?},", stack_frame);

    hit_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PIC.lock()
            .notify_end_of_interrupt(HardwareInterruptIndex::Timer.as_u8());
    };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut keyboard_metadata_port = Port::new(0x60);
    let scan_code: u8 = unsafe { keyboard_metadata_port.read() };

    if let Ok(Some(event)) = keyboard.add_byte(scan_code) {
        match keyboard.process_keyevent(event) {
            Some(DecodedKey::Unicode(cr)) => print!("{}", cr),
            Some(DecodedKey::RawKey(key)) => print!("{:?}", key),
            _ => {}
        }
    };

    unsafe {
        PIC.lock()
            .notify_end_of_interrupt(HardwareInterruptIndex::Keyboard.as_u8());
    };
}

#[cfg(test)]
mod test_interrupt_descriptor_table {
    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
}
