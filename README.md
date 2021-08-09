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

## Project Architecture
RhubarbOS is designed as a microkernel -- the actual OS-level code running is
minimal. On top of this minimal layer (which implements processes, virtual
memory, threads, and some permissions), other parts of the OS are written as
regular processes.

## License
RhubarbOS is licensed as open source software under the MIT License. See
[LICENSE](./LICENSE) for details.
