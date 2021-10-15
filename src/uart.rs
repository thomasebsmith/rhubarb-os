// uart
// ---
// Code for printing to UART serial output.

// An address that outputs to UART serial output when written to.
static UART_MMIO_ADDR: i64 = 0x3F201000;

use core::fmt;

unsafe fn print_char(ch: char) {
    core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
}

// Prints a string to UART using MMIO.
pub unsafe fn print_str(string: &str) {
    for ch in string.chars() {
        print_char(ch)
    }
}

// A Writer for UART using MMIO.
struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        unsafe {
            print_str(string)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // FIXME: This is not thread-safe.
    use fmt::Write;
    Writer{}.write_fmt(args).unwrap();
}

// Prints to UART using MMIO.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

// Prints a line to UART using MMIO.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
