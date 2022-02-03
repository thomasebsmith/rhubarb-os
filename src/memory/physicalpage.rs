// physicalpage
// ---
// Represents a page stored in physical memory.

pub struct DiskBlock(pub u64);
pub struct PhysicalPageNumber(pub u64);

pub const usize PAGE_SIZE = 4 * 1024;

pub unsafe fn pointer_to(ppn: PhysicalPageNumber) -> *mut u8 {
    return (ppn * PAGE_SIZE) as *mut u8;
}
