// virtualpage
// ---
// Represents a virtual page of memory.

use crate::process::ProcessId;
use physicalpage::{DiskBlock, PhysicalPageNumber};

pub struct VirtualPageNumber(pub u64);

pub enum VirtualPageLocation {
    // Used for pages stored in swap space
    Evicted { diskBlock: DiskBlock },

    // Used for pages stored in memory
    Resident { physicalPageNumber: PhysicalPageNumber },
}

pub struct VirtualPage {
    // The process that owns this page. Pages cannot be shared at the user level
    pub owner: ProcessId,

    // Where this page is currently stored
    pub location: VirtualPageLocation,

    // Whether this page can be modified
    pub writeEnabled: bool,

    // Whether this page can be executed as machine code
    pub executeEnabled: bool,

    // How recently used this page was used. Lower is more recent
    pub recent_use_bits: u8,
}

pub struct ProcessMemory {
    // How many virtual pages a process is using
    pub virtual_pages_in_use: u64;
}
