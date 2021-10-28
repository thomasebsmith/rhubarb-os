// arch
// ---
// Imports architecture-specific code (e.g. boot assembly).

// Currently, only 64-bit ARM systems (aarch64) are supported.

// aarch64
#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
