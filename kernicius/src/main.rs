#![no_std]
#![no_main]

use bootloader_api::{BootInfo, info::FrameBuffer};

#[unsafe(no_mangle)]
pub fn kernel_entry(boot_info: &'static mut BootInfo) -> ! {
    let Some(_) = boot_info.framebuffer.as_mut().map(FrameBuffer::buffer_mut) else {
        panic!("No framebuffer :(");
    };

    loop {}
}

bootloader_api::entry_point!(kernel_entry);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
