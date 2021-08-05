use crate::arch::asm::wait_forever;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    wait_forever()
}
