mod framebuffer;
use crate::framebuffer::Framebuffer;

use std::time::Instant;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("USAGE: cfb (record|compile) <FILE>");
        std::process::exit(2);
    }
    let query = &args[1];
    let outpath = &args[2];

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
    for _ in 1..=60 /*loop*/ {
        let start = Instant::now();
        let mut framebuffer = File::open(&fbpath)?;
        buffer.clear();
        framebuffer.read_to_end(&mut buffer)?;
        outfile.write_all(&buffer)?;

        println!("TIME: {:?}", start.elapsed().as_secs_f32());
        outfile.write_all(&start.elapsed().as_secs_f32().to_le_bytes())?;
    }

    // Append time, number of frames? 

    Ok(())
}

