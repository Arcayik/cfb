mod parse;

use framebuffer::Framebuffer;

use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("USAGE: cfb (record|compile) <FILE>");
        std::process::exit(2);
    }
    let query = &args[1];
    let outpath = &args[2];

    // Initialize Framebuffer struct
    let framebuffer = Framebuffer::new("/dev/fb0").unwrap();
    let w = framebuffer.var_screen_info.xres;
    let h = framebuffer.var_screen_info.yres;
    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    // Create output file
    let mut outfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outpath)?;

    // Write File Header
    let header: &[u8] = &[w.to_le_bytes(), h.to_le_bytes(), bytespp.to_le_bytes()].concat();
    
    outfile.write_all(&header)?;

    // Initialize memory buffer
    let mut frame: Vec<u8> = vec![0u8; (line_length * h) as usize];
    
    // Loop to collect frame data as fast as possible
    for _ in 1..=15 {
        let start = Instant::now();
        frame.clear();
        let frame = framebuffer.read_frame();

        outfile.write_all(&frame)?;

        let time: f32 = start.elapsed().as_secs_f32();
        println!("TIME: {:?}", time);
        outfile.write_all(&time.to_le_bytes())?;
    }

    let capture = parse::CaptureFile::from_path(outpath);
    dbg!(&capture.width, &capture.height, &capture.depth);
    capture.save_frames_as_png()?;

    // Append time, number of frames? 
    Ok(())
}

