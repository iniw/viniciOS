use clap::Parser;
use std::{env, fs, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let uefi_image_path = env!("UEFI_PATH");
    let kernel_image_path = env!("KERNEL_PATH");

    // The bootloader crate adds some hash stuff to the output directory of the kernel image,
    // essentially randomizing it's path, so we copy it over to a saner and more predictable one:
    // 'target/{profile}/kernicius'.
    // This is to help with debugging, since our debugger needs to load the kernel image to
    // retrieve the debug symbols. See `debug.lldb` for an example.
    let current_exe = env::current_exe().unwrap();
    let kernel_copy = current_exe.with_file_name("kernicius");
    fs::copy(&kernel_image_path, &kernel_copy).unwrap();

    let mut cmd = Command::new("qemu-system-x86_64");

    if args.debug {
        // -s: Start gdb server @ localhost:1234
        // -S: Freeze CPU on startup
        cmd.arg("-s").arg("-S");
    }

    cmd.arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={uefi_image_path}"));

    cmd.spawn()
        .expect("Failed to spawn qemu process")
        .wait()
        .expect("Failed to wait for qemu to exit");
}
