# RhubarbOS
RhubarbOS is a toy operating system for Raspberry Pi computers, written in Rust.

## Prerequisites
- A nightly rust toolchain (needed to support `asm!` macros).
- (Recommended) QEMU (to run the operating system).

## Quick Start
```sh
$ git clone https://github.com/thomasebsmith/rhubarb-os.git
$ cd rhubarb-os/
$ cargo build --release
$ qemu-system-aarch64 -M raspi3 -serial stdio -display none -kernel target/aarch64-unknown-none-softfloat/release/os
^C
```
