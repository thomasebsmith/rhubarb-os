// virtualpage
// ---
// Represents a virtual page of memory.

use crate::process::ProcessId;

pub struct DiskBlock(pub u64);
pub struct PhysicalPageNumber(pub u64);
pub struct VirtualPageNumber(pub u64);

pub enum VirtualPageLocation {
    // Used for pages stored in swap space
    Evicted { diskBlock: DiskBlock },

    // Used for pages stored in memory
    Resident { physicalPageNumber: PhysicalPageNumber },
}

pub struct VirtualPage {
    pub owner: ProcessId,
    pub location: VirtualPageLocation,
    pub writeEnabled: bool,
    pub executeEnabled: bool,
}
