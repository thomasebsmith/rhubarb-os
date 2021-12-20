use core::arch::global_asm;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn _start_os() -> ! {
    crate::main()
}
