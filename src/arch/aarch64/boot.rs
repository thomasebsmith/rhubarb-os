use core::arch::global_asm;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn _start_os(cpu_id: i64) -> ! {
    crate::main(cpu_id)
}
