#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]

global_asm!(include_str!("boot.s"));

fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe")
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    wait_forever()
}

static UART_MMIO_ADDR: i64 = 0x3F201000;

#[no_mangle]
pub fn _start_os() -> ! {
    let string = "Hello, world!\n";
    for ch in string.chars() {
        unsafe {
            core::ptr::write_volatile(UART_MMIO_ADDR as *mut u8, ch as u8);
        }
    }
    wait_forever()
}
