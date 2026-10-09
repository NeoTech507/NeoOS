use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo must set OUT_DIR"));
    let kernel = PathBuf::from(
        std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel")
            .expect("Cargo must build the kernel artifact"),
    );
    let bios_path = out_dir.join("neoos-bios.img");

    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("failed to create the NeoOS BIOS image");

    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
