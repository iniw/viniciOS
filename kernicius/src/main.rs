#![no_std]
#![no_main]

use bootloader_api::{BootInfo, info::FrameBuffer};

pub fn kernel_entry(boot_info: &'static mut BootInfo) -> ! {
    let Some(framebuffer) = boot_info.framebuffer.as_mut().map(FrameBuffer::buffer_mut) else {
        panic!("No framebuffer :(");
    };

    framebuffer.fill(0x55);

    loop {}
}

bootloader_api::entry_point!(kernel_entry);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
