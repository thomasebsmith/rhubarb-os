// virtualpage
// ---
// Represents a virtual page of memory.

use crate::process::ProcessId;

pub struct VirtualPageLocation {
    // TODO
}

pub struct VirtualPage {
    pub owner: ProcessId,
    pub location: VirtualPageLocation,
    pub writeEnabled: bool,
    pub executeEnabled: bool,
}
