#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {$crate::serial::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print("\n")
    };
    ($($arg:tt)*) => {$crate::serial_print!("{}\n", format_args!($($arg)*))};
}
