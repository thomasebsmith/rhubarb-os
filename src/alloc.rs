// Only 64-bit architectures are currently supported.
pub u64 address_bits = 64;

// TODO: Verify that this is correct.
pub u64 page_size = 4 * 1024;

// TODO: This will need to be more complicated - the actual set of available
//  pages will need to be tracked.
pub u128 available_pages = 0; // TODO
