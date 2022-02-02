// physicalpage
// ---
// Represents a page stored in physical memory.

pub struct DiskBlock(pub u64);
pub struct PhysicalPageNumber(pub u64);

pub unsafe fn pointer_to(ppn: PhysicalPageNumber) -> *u8 {
    // TODO
}
