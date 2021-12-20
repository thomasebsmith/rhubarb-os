// asm
// ---
// Exposes assembly-level, processor-specific functionality using a generic API.

use core::arch::asm;

/// Waits forever (in an efficient manner). This does not busy-wait; rather,
/// it repeatedly waits for interrupts or events.
pub fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe")
        }
    }
}
