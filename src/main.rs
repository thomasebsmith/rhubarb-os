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

unsafe fn print_char_uart(ch: char) {
    core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
}

unsafe fn print_uart(string: &str) {
    for ch in string.chars() {
        print_char_uart(ch)
    }
}

unsafe fn print_u32_uart(num: u32, radix: u32) {
    if radix <= 1 {
        panic!("Radix must be > 1")
    }
    if radix > 36 {
        panic!("Radix must be <= 36")
    }

    let mut to_print = num;
    let mut radix_offset: u32 = 1;
    while radix_offset < to_print / radix {
        radix_offset *= radix;
    }

    while radix_offset >= 1 {
        let digit = to_print / radix_offset;
        to_print %= radix_offset;
        radix_offset /= radix;
        print_char_uart(core::char::from_digit(digit, radix).unwrap());
    }
}

unsafe fn print_ptr_uart<T>(ptr: *const T) {
    let nullptr = 0x0 as *const T;
    let pointer_size = ptr.offset_from(nullptr);
    print_u32_uart(pointer_size as u32, 16);
}

// This is the Rust entry point for the OS. It should not halt except when a
// fatal error occurs (i.e. the OS panics).
pub fn main() -> ! {
    let x = 0;
    unsafe {
        print_uart("Hello, world!\n");
        print_uart("Stack variable address: ");
        print_ptr_uart(core::ptr::addr_of!(x));
        print_char_uart('\n');
        print_uart("---\n");
    }
    arch::asm::wait_forever()
}
