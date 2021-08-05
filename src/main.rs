#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]

mod arch;
mod panic;

static UART_MMIO_ADDR: i64 = 0x3F201000;

pub fn main() -> ! {
    let string = "Hello, world!\n";
    for ch in string.chars() {
        unsafe {
            core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
        }
    }
    arch::asm::wait_forever()
}
