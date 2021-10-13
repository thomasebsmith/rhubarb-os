// main
// ---
// The driver of the OS. After boot, the OS starts here.

#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]

mod arch;
mod panic;
mod uart;

// This is the Rust entry point for the OS. It should not halt except when a
// fatal error occurs (i.e. the OS panics).
pub fn main() -> ! {
    let x = 0;
    unsafe {
        uart::print_str("Hello, world!\n");
        uart::print_str("Stack variable address: ");
        uart::print_ptr(core::ptr::addr_of!(x));
        uart::print_str("\n---\n");
    }
    arch::asm::wait_forever()
}
