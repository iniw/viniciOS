use std::process::Command;

fn main() {
    let kernel_image_path = env!("UEFI_PATH");

    Command::new("qemu-system-x86_64")
        .arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={kernel_image_path}"))
        .spawn()
        .expect("Failed to spawn qemu process")
        .wait()
        .expect("Failed to wait for qemu to exit");
}
