# RhubarbOS
RhubarbOS is a toy operating system for Raspberry Pi computers, written in Rust.

## Prerequisites
- A nightly rust toolchain (needed to support `asm!` and `global_asm!` macros).
- (Recommended) QEMU (to run the operating system).

## Quick Start
```sh
$ git clone https://github.com/thomasebsmith/rhubarb-os.git
$ cd rhubarb-os/
$ rustup target add aarch64-unknown-none-softfloat
$ cargo build --release
$ qemu-system-aarch64 -M raspi3 -serial stdio -display none -kernel target/aarch64-unknown-none-softfloat/release/os
^C
```

## Project Architecture
RhubarbOS is designed as a microkernel &mdash; the actual OS-level code running
is minimal. On top of this minimal layer (which implements processes, virtual
memory, threads, and some permissions), other parts of the OS are written as
regular processes.

## Project Roadmap
### v0.1 &mdash; In Progress
- Microkernel
- Processes, virtual memory, and threading support

### v0.2
- File system support
- Still no support for user-level programs

### v0.3
- User interface support (GUIs and CLIs)
- Support for custom drivers
- Support for user-level programs

## License
RhubarbOS is licensed as open source software under the MIT License. See
[LICENSE](./LICENSE) for details.
