use core::fmt::{self, Write};

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // 0x3F8 是 uart_16550 第一个串行接口的标准端口号
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init();
        Mutex::new(port)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).unwrap();
}
