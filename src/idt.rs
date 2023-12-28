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
use x86_64;
