use bootloader::UefiBoot;
use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("uefi.img");

    // set by cargo's artifact dependency feature, see
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel_path = PathBuf::from(env::var_os("CARGO_BIN_FILE_KERNICIUS_kernicius").unwrap());

    UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .expect("Failed to create UEFI disk image");

    // pass the disk image paths as env variables to the `main.rs`
    println!(
        "cargo:rustc-env=UEFI_PATH={}",
        uefi_path
            .as_os_str()
            .to_str()
            .expect("Invalid characters found in uefi path")
    );
    println!(
        "cargo:rustc-env=KERNEL_PATH={}",
        kernel_path
            .as_os_str()
            .to_str()
            .expect("Invalid characters found in kernel path")
    );
}
