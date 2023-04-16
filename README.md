# RhubarbOS
RhubarbOS is an operating system for Raspberry Pi computers, written in Rust. It
is intended to be a lightweight, secure, and customizable alternative to
Raspberry Pi OS and other general-purpose OSs that support the Pi.

## Prerequisites
- A nightly Rust toolchain
- [Recommended] QEMU (in order to emulate the operating system).
  Alternatively, an actual Raspberry Pi (to run the operating system).

  Note: RhubarbOS has not yet been tested on a physical Raspberry Pi.

## Quick Start
```sh
# To clone this repository, build a release binary, and emulate the OS in QEMU:
$ git clone https://github.com/thomasebsmith/rhubarb-os.git
$ cd rhubarb-os/
$ rustup target add aarch64-unknown-none-softfloat
$ cargo build --release
$ qemu-system-aarch64 -M raspi3b -serial stdio -display none -kernel target/aarch64-unknown-none-softfloat/release/os
...
^C
```

## Project Architecture
RhubarbOS is designed as a microkernel &mdash; the actual OS-level code running
is minimal. On top of this minimal layer, which implements processes, virtual
memory, threads, and some permissions, other parts of the OS are written as
regular processes with fairly broad permissions.

To support this design, RhubarbOS's scheduler, permission checks, and IPC code
will be highly optimized.

### Security
RhubarbOS is also designed to be secure. By default, processes have only
minimal abilities. They must be granted permissions to perform most system
calls, including all file system operations. These permissions must be
explicitly granted by an actual user.

Permissions are stored on disk, and since they are so integral to RhubarbOS's
security model, the disk must be protected. Disk encryption is enabled by
default, and the boot disk is protected from drivers and other low-level
software. In higher-level software, permissions are unmodifiable except via
explicit user action.

RhubarbOS's multiprocess design naturally protects against some security
vulnerabilities. For example, the Meltdown attack does not work since
RhubarbOS's processes cannot have other processes' memory mapped.

All APIs are exposed using low-overhead syscalls. Each syscall is handled by the
parent process that launched the current program. Most user-run programs will
have a native subprocess that is part of RhubarbOS as their parent process.

### Modularity and Customizability
Any program that has read access to another executable can run that executable
within a custom context. Any APIs can be replaced with custom implementations,
allowing users to introspect and modify any program.

### Interprocess Communication
Processes can voluntarily share information via high-level IPC APIs. A process
can configure its IPC API to accept communication from a specific process or
from any process.

Other processes must be identified using cryptographic certificates, which are
validated using a certificate authority system similar to the internet's.

IPCs are also low-overhead. They are designed to consume minimal CPU and memory.

For security reasons, processes cannot share memory directly.

### User Interface
RhubarbOS is designed to be used via its CLI and GUI. Both are equally capable.
The GUI is the default UI for new installations.

Programs should be designed to expose their own CLIs and GUIs.

### Drivers
A limited set of drivers for common Raapberry Pi peripherals is built in. More
drivers are available through the marketplace. Official marketplace drivers are
automatically suggested for attached peripherals.

Drivers are restricted so that they can only control the input from or output to
a single device (or multiple of the same device). However, even with this
restriction, malicious drivers may still be capable of damaging or taking over a
system.

## Project Roadmap
### v0.1 &mdash; In Progress
*Planned release: July 2023*
- Microkernel
- Processes
- Virtual memory
- Threading and concurrency primitives

### v0.2 &mdash; In Design Stage
*Planned release: December 2023*
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
- Official marketplace for drivers and apps
- Automatic suggestions of official drivers for somewhat common Raspberry Pi
  peripherals

### v0.6
- Detailed installation guide for supported Raspberry Pi computers
- OS more accessible for non-technical users
- Accessibility and troubleshooting modes

### Later Versions
After v0.6, future 0.x (beta) versions may be released to fix bugs or add new
features. Once a basic set of features is implemented and the OS is fully
usable, the first stable version, v1.0, will be released.

## Contributing
To contribute to this project, please find an issue to work on or open one based
on what you want to improve in the OS. Then, create a corresponding PR. Your PR
will be reviewed and hopefully approved and merged.

## License
RhubarbOS is licensed as open source software under the MIT License. See
[LICENSE](./LICENSE) for details.

Third party software was used as a reference in the making of this project.
See [LICENSE-THIRDPARTY](./LICENSE-THIRDPARTY) for details about that software.
All third party software that was used is open source and is compatible with the
MIT License.

## Copyright
Unless otherwise specified, all content in this repository is copyright &copy;
2021-2023 Thomas Smith.
