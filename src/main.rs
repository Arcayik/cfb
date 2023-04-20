mod framebuffer;
use crate::framebuffer::Framebuffer;

use std::time::Instant;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    let query = std::env::args().nth(1)
        .expect("No arguments given");
    let outpath = std::env::args().nth(2)
        .expect("No output file name given");

    // Initialize Framebuffer struct
    let fb = Framebuffer::from_device("fb0");

    // Initialize memory buffer
    let mut buffer = Vec::with_capacity(fb.get_mem_size());
    // Create output file
    let mut outfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outpath)?;

    // Write File Header
    let header: &[u8] = &[
        &fb.get_width().to_le_bytes()[..],
        &fb.get_height().to_le_bytes()[..],
        &fb.get_depth().to_le_bytes()[..],
    ].concat();
    
    outfile.write_all(&header)?;

    let fbpath = fb.get_path();

    // Loop to collect frame data as fast as possible
    for _ in 1..=10 /*loop*/ {
        let start = Instant::now();
        let mut framebuffer = File::open(&fbpath)?;
        buffer.clear();
        framebuffer.read_to_end(&mut buffer)?;
        outfile.write_all(&buffer)?;

        println!("TIME: {:?}", start.elapsed().as_secs_f32());
        outfile.write_all(&start.elapsed().as_secs_f32().to_le_bytes())?;
    }

    // Append time, frame number? 

    Ok(())
}

