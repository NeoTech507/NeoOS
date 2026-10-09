use std::process::{Command, ExitCode};

const BIOS_IMAGE: &str = env!("BIOS_PATH");

fn main() -> ExitCode {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "bios".to_owned());
    if mode != "bios" {
        eprintln!("Usage: cargo run -- [bios]");
        return ExitCode::from(2);
    }

    println!("Launching NeoOS in QEMU (BIOS). Press Ctrl+C to stop.");
    let status = Command::new("qemu-system-x86_64")
        .args(["-serial", "stdio", "-display", "none", "-no-reboot", "-m", "512M"])
        .arg("-drive")
        .arg(format!("format=raw,file={BIOS_IMAGE}"))
        .status();

    match status {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(error) => {
            eprintln!("Could not start QEMU: {error}");
            eprintln!("Install QEMU and make sure qemu-system-x86_64 is on PATH.");
            ExitCode::FAILURE
        }
    }
}
