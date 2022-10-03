// main
// ---
// The driver of the OS. After boot, the OS starts here.

#![no_main]
#![no_std]
#![feature(negative_impls)]

mod arch;
mod panic;
mod process;
mod sync;
mod thread;
mod uart;

use arch::asm::{get_cpu_id, wait_forever};

pub fn init() {
    if get_cpu_id() != 0 {
        wait_forever();
    }

    println!("Initializing on core {}", get_cpu_id());
}

/// The Rust entry point for the OS. It should not halt except when a
/// fatal error occurs (i.e. the OS panics) or the computer shuts down.
pub fn main() -> ! {
    println!("---");
    println!("Hello, world!");

    println!("Dynamic CPU ID: {}", get_cpu_id());

    let x = 0;
    println!("Stack variable address: {:p}", &x);

    println!("PID: {}", process::Process::get_current().id);
    println!("TID: {}", thread::Thread::get_current().id);
    {
        let m = sync::SpinLock::new(3);
        println!("Got value {} from lock", *m.try_lock().unwrap());
    }
    println!("---");

    wait_forever()
}
