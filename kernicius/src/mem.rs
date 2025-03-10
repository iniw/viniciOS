use bootloader_api::{
    BootInfo,
    info::{MemoryRegion, MemoryRegionKind},
};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{FrameAllocator, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB},
};

mod heap;

pub fn init(boot_info: &mut BootInfo) {
    let physical_memory_offset = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("Physical memory wasn't mapped. Check bootloader config."),
    );

    let kernel_addr = VirtAddr::new(boot_info.kernel_image_offset);

    let level_4_page_table = unsafe { level_4_page_table(physical_memory_offset) };

    let mut mapper = unsafe { OffsetPageTable::new(level_4_page_table, physical_memory_offset) };
    let mut frame_allocator = BumpFrameAllocator::from_regions(&boot_info.memory_regions);

    let heap_start = (kernel_addr + boot_info.kernel_len).align_up(Page::<Size4KiB>::SIZE);
    let heap_end = heap_start + boot_info.kernel_len - 1;

    heap::init(heap_start, heap_end, &mut frame_allocator, &mut mapper);
}

/// SAFETY: `physical_memory_offset` must be a valid virtual memory offset containing the mapped
/// physical memory
unsafe fn level_4_page_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (physical_addr, _) = Cr3::read();
    let virtual_addr = physical_memory_offset + physical_addr.start_address().as_u64();
    unsafe { &mut *virtual_addr.as_mut_ptr() }
}

struct BumpFrameAllocator<Frames> {
    usable_frames: Frames,
}

impl BumpFrameAllocator<()> {
    fn from_regions(
        regions: &[MemoryRegion],
    ) -> BumpFrameAllocator<impl Iterator<Item = PhysFrame>> {
        let usable_frames = regions
            .iter()
            .filter_map(|r| match r.kind {
                MemoryRegionKind::Usable => {
                    let start = PhysFrame::containing_address(PhysAddr::new(r.start));
                    let end = PhysFrame::containing_address(PhysAddr::new(r.end));
                    Some(PhysFrame::range(start, end))
                }
                _ => None,
            })
            .flatten();

        BumpFrameAllocator { usable_frames }
    }
}

unsafe impl<Frames: Iterator<Item = PhysFrame>> FrameAllocator<Size4KiB>
    for BumpFrameAllocator<Frames>
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.usable_frames.next()
    }
}
