use crate::sync::ThreadUnsafeGlobal;
use bootloader_api::{BootInfo, info::FrameBuffer};
use core::{
    ascii,
    cell::{OnceCell, RefCell},
    ptr::NonNull,
};

mod bitmaps;

#[derive(Debug)]
struct Logger {
    framebuffer: NonNull<FrameBuffer>,
    column: usize,
    line: usize,
    max_lines: usize,
}

const SCALING: usize = 3;
const SPACING: usize = SCALING;

impl Logger {
    fn new(framebuffer: &mut FrameBuffer) -> Self {
        // Clear screen
        framebuffer.buffer_mut().fill(0x0);
        let info = framebuffer.info();

        Self {
            framebuffer: NonNull::from(framebuffer),
            column: 0,
            line: 0,
            max_lines: info.height / ((bitmaps::SIZE + 1) * SCALING),
        }
    }

    fn framebuffer(&mut self) -> &mut FrameBuffer {
        unsafe { self.framebuffer.as_mut() }
    }

    fn newline(&mut self) {
        self.column = 0;
        self.line = self.line + 1;
        if self.line >= self.max_lines {
            self.line = self.max_lines - 1;

            let line = self.line;
            let max_lines = self.max_lines;
            let info = self.framebuffer().info();
            let bitmap = self.framebuffer().buffer_mut();
            let line_to_offset = (bitmaps::SIZE + 1) * SCALING * info.stride * info.bytes_per_pixel;

            for line in 1..max_lines {
                let begin = line * line_to_offset;
                let end = (line + 1) * line_to_offset;
                let previous_begin = (line - 1) * line_to_offset;
                bitmap.copy_within(begin..end, previous_begin);
            }

            let begin = line * line_to_offset;
            bitmap[begin..].fill(0x0);
        }
    }

    fn render_char(&mut self, char: char) {
        let bitmap = match char.as_ascii().map(ascii::Char::to_u8) {
            Some(c) => match c {
                b'a'..=b'z' => bitmaps::LATIN_ALPHABET[(c - b'a') as usize],
                b'A'..=b'Z' => bitmaps::LATIN_ALPHABET[(c - b'A') as usize],
                b'0'..=b'9' => bitmaps::NUMBERS[(c - b'0') as usize],
                b'\t' | b' ' => bitmaps::BLANK_BITMAP,
                b'\n' => {
                    self.newline();
                    return;
                }
                _ => bitmaps::FALLBACK_BITMAP,
            },
            _ => bitmaps::FALLBACK_BITMAP,
        };

        let column = self.column;
        let info = self.framebuffer().info();
        let buffer = {
            let line = self.line;
            let bitmap = self.framebuffer().buffer_mut();
            &mut bitmap[line * (bitmaps::SIZE + 1) * SCALING * info.stride * info.bytes_per_pixel..]
        };

        for (i, row) in bitmap.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                let color = match pixel {
                    false => 0x0,
                    true => 0xFF,
                };

                for vertical_fill in 0..SCALING {
                    let line_offset = (vertical_fill + i * SCALING) * info.stride;
                    let line = &mut buffer[line_offset * info.bytes_per_pixel..];

                    let begin = column + j * SCALING;
                    let end = SCALING;
                    line[begin * info.bytes_per_pixel..][..end * info.bytes_per_pixel].fill(color);
                }
            }
        }

        self.column += bitmaps::SIZE * SCALING + SPACING;
    }

    fn render_str(&mut self, string: &str) {
        for c in string.chars() {
            self.render_char(c);
        }
    }
}

static LOGGER: ThreadUnsafeGlobal<OnceCell<RefCell<Logger>>> =
    unsafe { ThreadUnsafeGlobal::new(OnceCell::new()) };

pub fn init(boot_info: &mut BootInfo) {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        LOGGER
            .set(RefCell::new(Logger::new(framebuffer)))
            .expect("`log::init` should only be called once");
    }
}

pub fn print(message: &str) {
    if let Some(logger) = LOGGER.get() {
        let mut logger = logger.borrow_mut();
        logger.render_str(message);
    }
}

pub fn println(message: &str) {
    if let Some(logger) = LOGGER.get() {
        let mut logger = logger.borrow_mut();
        logger.render_str(message);
        logger.render_char('\n');
    }
}

macro_rules! info {
    ($msg:expr) => {
        $crate::log::println($msg);
    };
    ($fmt:expr, $($arg:tt)+) => {{
        let msg = ::alloc::format!($fmt, $($arg)+);
        $crate::log::println(&msg);
    }};
}

pub(crate) use info;
