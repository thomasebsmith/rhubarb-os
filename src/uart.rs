// uart
// ---
// Code for printing to UART serial output.

use core::fmt;
use crate::sync::SpinLock;

// A mutex protecting an address that outputs to UART serial output when
// written to.
static UART_MMIO_ADDR_LOCK: SpinLock<i64> = SpinLock::new(0x3F201000 as i64);

// Writes a single character to addr. Not thread-safe.
unsafe fn print_char(ch: char, addr: i64) {
    core::ptr::write_volatile(addr as *mut u8, ch as u8);
}

/// Prints a string to UART using MMIO.
///
/// # Arguments
///
/// * `string` - The string to print
pub unsafe fn print_str(string: &str) {
    let guard = UART_MMIO_ADDR_LOCK.lock();
    for ch in string.chars() {
        print_char(ch, *guard)
    }
}

/// A struct that conforms to fmt::write and writes to UART via MMIO.
struct Writer {}

impl fmt::Write for Writer {
    /// Writes a string to UART.
    ///
    /// # Arguments:
    ///
    /// * `string` - The string to write
    ///
    /// # Return value:
    /// Whether the write succeeded
    fn write_str(&mut self, string: &str) -> fmt::Result {
        unsafe {
            print_str(string)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // This function is used internally to implement print! and println!.
    use fmt::Write;
    // It's OK to unwrap the result of write_fmt here.
    // If formatting fails, unwrap will panic, which is the desired behavior of
    //  print! and println!.
    Writer{}.write_fmt(args).unwrap();
}

/// Prints formatted argument(s) to UART using MMIO.
///
/// # Arguments
///
/// * `arg` - A value compatible with `format_args!`
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

/// Prints formatted argument(s) to UART using MMIO, along with a new line
/// character.
///
/// # Arguments
///
/// * `arg` - A value compatible with `format_args!`
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
