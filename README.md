# RhubarbOS
RhubarbOS is a toy operating system for Raspberry Pi computers, written in Rust.

## Prerequisites
- A nightly Rust toolchain (needed to support `asm!` and `global_asm!` macros).
- [Recommended] QEMU (to run the operating system).

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

To support this design, RhubarbOS's scheduler and IPC code will be highly
optimized.

RhubarbOS is also designed to be secure. By default, processes have only
minimal abilities. They must be granted permissions to perform most system
calls, including all file system operations.

Permissions are stored on disk, and since they are so integral to RhubarbOS's
security model, the disk must be protected. Disk encryption is enabled by
default, and the boot disk is protected from drivers and other low-level
software.

RhubarbOS's multiprocess design naturally protects against some security
vulnerabilities. For example, the Meltdown attack should not work since
RhubarbOS's processes do not have other processes' memory mapped.

## Project Roadmap
### v0.1 &mdash; In Progress
- Microkernel
- Processes, virtual memory, and threading support

### v0.2
- File system support (including secure permissions)
- Still no support for user-level programs

### v0.3
- User interface support (GUIs and CLIs)
- Support for custom drivers
- Support for user-level programs

### v0.4
- Drivers for common Raspberry Pi peripherals included in the OS
- Basic set of built-in apps included in the OS

### v0.5
- Official store for drivers and apps
- OS more accessible for non-technical users

### Later Versions
After v0.4, future 0.x versions may be released to fix bugs or add new features.
Once a basic set of features is implemented, the first stable version, v1.0,
will be released.

## License
RhubarbOS is licensed as open source software under the MIT License. See
[LICENSE](./LICENSE) for details.

Some third party software was used as a reference or in the making of this
project. See [LICENSE-THIRDPARTY](./LICENSE-THIRDPARTY) for details.

## Copyright
Unless otherwise specified, all code is copyright &copy; 2021 Thomas Smith.
