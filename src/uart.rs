// uart
// ---
// Code for printing to UART serial output.

// An address that outputs to UART serial output when written to.
static UART_MMIO_ADDR: i64 = 0x3F201000;

pub unsafe fn print_char(ch: char) {
    core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
}

pub unsafe fn print_str(string: &str) {
    for ch in string.chars() {
        print_char(ch)
    }
}

pub unsafe fn print_u32(num: u32, radix: u32) {
    if radix <= 1 {
        panic!("Radix must be > 1")
    }
    if radix > 36 {
        panic!("Radix must be <= 36")
    }

    let mut to_print = num;
    let mut radix_offset: u32 = 1;
    while radix_offset < to_print / radix {
        radix_offset *= radix;
    }

    while radix_offset >= 1 {
        let digit = to_print / radix_offset;
        to_print %= radix_offset;
        radix_offset /= radix;
        print_char(core::char::from_digit(digit, radix).unwrap());
    }
}

pub unsafe fn print_ptr<T>(ptr: *const T) {
    let nullptr = 0x0 as *const T;
    let pointer_size = ptr.offset_from(nullptr);
    print_u32(pointer_size as u32, 16);
}

