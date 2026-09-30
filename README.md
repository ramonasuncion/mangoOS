# mangoOS

A hobby osdev project. I intend to develop a x64 monolithic OS.

> [!NOTE]
> Once [Ripe](https://github.com/ramonasuncion/ripe), a systems language I'm building, is mature enough, I plan to rewrite mangoOS in it.

## Demo

![OS](./os_demo.png)

## Building

You need an x86_64 Linux host with `gcc`, `binutils`, `make`, `nasm`, `xorriso`, `git`, `curl` and `qemu-system-x86_64`. The Makefile fetches [Limine](https://github.com/limine-bootloader/limine) on first build.

```bash
cd src
make all       # Build kernel (creates build/kernel.elf)
make iso       # Create bootable ISO image
make run       # Build and run in QEMU
make debug     # Build and run with GDB debugging
make clean     # Clean build artifacts
```

To debug, run `make debug` in one terminal and attach from another:

```bash
gdb build/kernel.elf -ex "target remote :1234"
```

## Resources

- [OSDev - GCC Cross Compiler](https://wiki.osdev.org/GCC_Cross-Compiler)
- [Limine Protocol](https://codeberg.org/Limine/limine-protocol)
- [Limine Repository](https://codeberg.org/Limine/Limine)
- [Debian - QEMU](https://wiki.debian.org/QEMU)

## Contributing

I welcome contributions to this project. Please read [CONTRIBUTING.md](./CONTRIBUTING.md).

## Reading List

1. Operating Systems: Design and Implementation (Second Edition).
   by Andrew S. Tanenbaum (Author), Albert S. Woodhull (Author) - Thanks to Professor Xiannong Meng at Bucknell University.
2. Operating System Concepts, 5th Edition
   by Abraham Silberschatz (Author), Baer Galvin (Author), Peter Gagne (Author) - Thanks to Professor Jessen Havill at Bucknell Univerisity.
