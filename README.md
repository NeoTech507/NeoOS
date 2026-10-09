# NeoOS

NeoOS is an original x86_64 hobby-kernel starter written in Rust. It boots through the Rust `bootloader` crate and reports its startup state over the first serial port (UART0). Its text interface uses a restrained, technology-forward system-console style.

The product ID shown by the demo kernel is `ABCD-6769-1479-AK47`. It is displayed as an identifier only; there is no license validation or enforcement.

This is a bootable kernel foundation, not a complete desktop operating system. It does not yet include graphics, input, storage, networking, or a userspace.

## Requirements

- Rustup with the nightly toolchain. The included `rust-toolchain.toml` selects nightly and installs the required `llvm-tools-preview` component and `x86_64-unknown-none` target.
- QEMU for Windows, with `qemu-system-x86_64` available on `PATH`.
- An internet connection for the first Cargo build to download Rust crates.

## Build and run

From this directory, run:

```powershell
cargo build
cargo run -- bios
```

QEMU displays the kernel's serial console in the terminal. Press Ctrl+C to stop the emulator.

## Project layout

- `kernel/`: `no_std`, `no_main` Rust kernel and boot entry point.
- `build.rs`: turns the compiled kernel into a BIOS boot image.
- `src/main.rs`: starts the image in QEMU.
