use bootloader_api::BootInfo;

const A: [[char; 5]; 5] = [
    ['.', '+', '+', '+', '.'],
    ['+', '.', '.', '.', '+'],
    ['+', '+', '+', '+', '+'],
    ['+', '.', '.', '.', '+'],
    ['+', '.', '.', '.', '+'],
];

pub fn init(boot_info: &mut BootInfo) {
    let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer :(");

    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    // Clear screen
    buffer.fill(0x0);

    const PIXEL_SCALE: usize = 5;

    let bpp = info.bytes_per_pixel;

    for (i, letter_line) in A.iter().enumerate() {
        for (j, letter_bit) in letter_line.iter().enumerate() {
            let color = match letter_bit {
                '.' => 0x0,
                '+' => 0xFF,
                _ => panic!(),
            };

            for fbi in 0..PIXEL_SCALE {
                let line_offset = (i * PIXEL_SCALE + fbi) * info.stride;
                let line = &mut buffer[line_offset * bpp..];

                let begin = j * PIXEL_SCALE;
                let end = PIXEL_SCALE;
                line[begin * bpp..][..end * bpp].fill(color);
            }
        }
    }
}
