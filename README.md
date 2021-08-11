# RhubarbOS
RhubarbOS is a toy operating system for Raspberry Pi computers, written in Rust.

## Prerequisites
- A nightly rust toolchain (needed to support `asm!` and `global_asm!` macros).
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

## Project Roadmap
### v0.1 -- In Progress
- Microkernel
- Support processes, virtual memory, and threading

### v0.2
- File system support
- Still no support for user-level programs

## License
RhubarbOS is licensed as open source software under the MIT License. See
[LICENSE](./LICENSE) for details.
