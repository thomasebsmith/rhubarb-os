use crate::arch::asm::wait_forever;

// The function to be called when a panic occurs. Right now, this halts the
// processor (stack unwinding and similar features are not yet implemented).
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    wait_forever()
}
