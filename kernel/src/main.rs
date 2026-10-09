#![no_std]
#![no_main]

use bootloader_api::{BootInfo, entry_point};
use core::fmt::Write;
use uart_16550::backend::PioBackend;
use uart_16550::{Config, Uart16550Tty};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let mut port = serial();
    let _ = writeln!(
        port,
        "\r\n+----------------------------------------------------------+\r\n\
         | N E O O S  //  CORE SYSTEM                            |\r\n\
         | x86_64  /  BIOS BOOT  /  STATUS: ONLINE               |\r\n\
         +----------------------------------------------------------+\r\n\
         | PRODUCT ID: ABCD-6769-1479-AK47                       |\r\n\
         | KERNEL: Rust no_std  /  UART0: READY                   |\r\n\
         +----------------------------------------------------------+\r\n\
         NeoOS kernel initialized. System halted safely.\r"
    );

    halt()
}

fn serial() -> Uart16550Tty<PioBackend> {
    unsafe { Uart16550Tty::new_port(0x3F8, Config::default()) }
        .expect("UART0 initialization failed")
}

fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(serial(), "\r\nNEOOS KERNEL PANIC: {info}");
    halt()
}
