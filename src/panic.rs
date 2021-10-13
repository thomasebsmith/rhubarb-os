// panic
// ---
// Code for handling kernel crashes. This shouldn't run much. :)

use crate::arch::asm::wait_forever;
use crate::println;

// The function to be called when a panic occurs. Right now, this halts the
// processor (stack unwinding and similar features are not yet implemented).
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    wait_forever()
}
