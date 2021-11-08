// panic
// ---
// Code for handling kernel crashes. This shouldn't run much. :)

use crate::arch::asm::wait_forever;
use crate::println;

/// Ends the OS (causing it to crash) after printing crash information
///
/// Currently halts the processor (stack unwinding and similar features are not
/// yet implemented).
///
/// # Arguments
///
/// * `info` - Information about the crash
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    wait_forever()
}
