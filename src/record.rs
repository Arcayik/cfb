use framebuffer::Framebuffer;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

pub fn capture(fbdev: &str, path: &str) -> Result<(), std::io::Error> {
    // Initialize Framebuffer struct
    let framebuffer = Framebuffer::new(fbdev).unwrap();
    let w = framebuffer.var_screen_info.xres;
    let h = framebuffer.var_screen_info.yres;
    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    // Create output file
    let mut outfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    // Write File Header
    let header: &[u8] = &[w.to_le_bytes(), h.to_le_bytes(), bytespp.to_le_bytes()].concat();
    outfile.write_all(&header)?;

    // Initialize memory buffer
    let mut frame: Vec<u8> = vec![0u8; (line_length * h) as usize];

    // Loop to collect frame data as fast as possible
    for i in 1..=30 {
        let start = Instant::now();
        frame.clear();
        let frame = framebuffer.read_frame();

        outfile.write_all(&frame)?;

        let time: f32 = start.elapsed().as_secs_f32();
        println!("{}: {:?}s", i, time);

        outfile.write_all(&time.to_le_bytes())?;
    }
    Ok(())
}
