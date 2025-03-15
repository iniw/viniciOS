use core::{
    alloc::GlobalAlloc,
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
};
extern crate alloc;

pub fn init(
    heap_start: VirtAddr,
    heap_end: VirtAddr,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) {
    ALLOCATOR.init(heap_start.as_mut_ptr(), heap_end.as_mut_ptr());

    let heap_base = Page::containing_address(heap_start);
    let heap_end = Page::containing_address(heap_end);
    for page in Page::range_inclusive(heap_base, heap_end) {
        let Some(frame) = frame_allocator.allocate_frame() else {
            // TODO: Log that there's no more physical memory available.
            break;
        };

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .unwrap()
                .flush();
        }
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

struct Allocator {
    heap_start: AtomicPtr<u8>,
    heap_end: AtomicPtr<u8>,
    cursor: AtomicPtr<u8>,
    counter: AtomicUsize,
}

impl Allocator {
    const fn new() -> Self {
        Self {
            heap_start: AtomicPtr::new(ptr::null_mut()),
            heap_end: AtomicPtr::new(ptr::null_mut()),
            cursor: AtomicPtr::new(ptr::null_mut()),
            counter: AtomicUsize::new(0),
        }
    }

    fn init(&self, heap_start: *mut u8, heap_end: *mut u8) {
        self.heap_start.store(heap_start, Ordering::Relaxed);
        self.heap_end.store(heap_end, Ordering::Relaxed);
        self.cursor.store(heap_start, Ordering::Relaxed);
        self.counter.store(0, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let bytes = layout.align() - 1 + layout.size();

        let with_bytes = unsafe { self.cursor.load(Ordering::Relaxed).byte_add(bytes) };
        if with_bytes > self.heap_end.load(Ordering::Relaxed) {
            return ptr::null_mut();
        }

        self.counter.fetch_add(1, Ordering::Relaxed);
        let cursor = self.cursor.fetch_byte_add(bytes, Ordering::Relaxed);
        unsafe { cursor.byte_add(cursor.align_offset(layout.align())) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        if self
            .counter
            .compare_exchange(1, 0, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            let heap_start = self.heap_start.load(Ordering::Relaxed);
            self.cursor.store(heap_start, Ordering::Relaxed);
        }
    }
}
