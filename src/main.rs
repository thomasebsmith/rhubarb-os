// main
// ---
// The driver of the OS. After boot, the OS starts here.

#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]

mod arch;
mod panic;

// An address that outputs to UART serial output when written to.
static UART_MMIO_ADDR: i64 = 0x3F201000;

// This is the Rust entry point for the OS. It should not halt except when a
// fatal error occurs (i.e. the OS panics).
pub fn main() -> ! {
    let string = "Hello, world!\n";
    for ch in string.chars() {
        unsafe {
            core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
        }
    }
    arch::asm::wait_forever()
}
