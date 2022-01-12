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

/// Returns the ID of the current CPU (from 0-3 inclusive on a Raspberry Pi).
pub fn get_cpu_id() -> u64 {
    let mpidr_el1: u64;
    unsafe {
        asm!("mrs {}, MPIDR_EL1", out(reg) mpidr_el1);
    }

    // The last two bits of MPIDR_EL1 distinguish the 4 CPU cores on a
    // Raspberry Pi.
    const CPU_CORE_AFFINITY_MASK: u64 = 0b11;

    return mpidr_el1 & CPU_CORE_AFFINITY_MASK;
}
