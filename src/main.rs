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

/// The Rust entry point for the OS. It should not halt except when a
/// fatal error occurs (i.e. the OS panics) or the computer shuts down.
pub fn main() -> ! {
    let x = 0;
    println!("---");
    println!("Hello, world!");
    println!("Stack variable address: {:p}", &x);
    println!("---");
    arch::asm::wait_forever()
}
