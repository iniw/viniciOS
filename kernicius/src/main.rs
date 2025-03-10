#![no_std]
#![no_main]
#![feature(strict_provenance_atomic_ptr)]

mod log;
mod mem;

extern crate alloc;

use alloc::vec::Vec;
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping};

#[unsafe(no_mangle)]
pub fn kernel_entry(boot_info: &'static mut BootInfo) -> ! {
    mem::init(boot_info);
    log::init(boot_info);

    let mut vec = Vec::with_capacity(20);
    for i in 0..10 {
        vec.push(i);
    }

    for i in 0..10 {
        assert_eq!(i, vec[i]);
    }

    drop(vec);

    loop {}
}

bootloader_api::entry_point!(kernel_entry, config = &bootloader_config());

const fn bootloader_config() -> BootloaderConfig {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
