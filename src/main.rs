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

unsafe fn print_uart(string: &str) {
    for ch in string.chars() {
        core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
    }
}

unsafe fn print_size_uart(num: isize, _base: isize) {
    if num < 0 {
        panic!("Printing negatives NYI")
    }
    print_uart("NYI")
}

unsafe fn print_ptr_uart<T>(ptr: *const T) {
    let nullptr = 0x0 as *const T;
    let pointer_size = ptr.offset_from(nullptr);
    print_size_uart(pointer_size, 16);
}

// This is the Rust entry point for the OS. It should not halt except when a
// fatal error occurs (i.e. the OS panics).
pub fn main() -> ! {
    let x = 0;
    unsafe {
        print_uart("Hello, world!\n");
        print_uart("Stack variable address: ");
        print_ptr_uart(core::ptr::addr_of!(x));
        print_uart("\n");
    }
    arch::asm::wait_forever()
}
