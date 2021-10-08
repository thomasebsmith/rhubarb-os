// alloc - Raw memory allocation
// ---
// Provides an interface for allocating physical (not virtual) memory.

// Only 64-bit architectures are currently supported.
pub u64 address_bits = 64;

// TODO: Verify that this is correct.
pub u64 page_size = 4 * 1024;

// TODO: This will need to be more complicated - the actual set of available
//  pages will need to be tracked.
//  One way to do this would be to divide all physmem into "megapages" (of size
//  ~16MB). The availability of megapages is tracked using the first 1+ physical
//  pages. Within each megapage, the first physical page would track
//  availability of regular pages (using bitsets).
pub u128 available_pages = 0; // TODO
