use std::fs::OpenOptions;
use std::io::Read;

#[derive(Debug)]
pub struct CaptureFile {
    pub height: u32,
    pub width: u32,
    pub depth: u32,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone)]
pub struct Frame{
    data: Vec<u8>,
    pub time: f32,
}

impl CaptureFile {
    pub fn from_path(capture: &str) -> CaptureFile {
        let mut data = Vec::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(capture).unwrap();
        file.read_to_end(&mut data)
            .expect("Failed to read capture file");
        
        // Get Height, Width, Depth
        let w = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let h = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let d = u32::from_le_bytes(data[8..12].try_into().unwrap());
        dbg!(&h, &w, &d);

        // Get Frame Size
        let framesize: usize = (h * w * d) as usize;
        dbg!(&framesize);
        
        // Read Frames
        let mut startaddr: usize = 12;
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            let endaddr = startaddr + framesize;
            if endaddr > data.len() { break }
            // get Frame
            let timestamp = f32::from_le_bytes(data[endaddr .. endaddr+4].try_into().unwrap());
            dbg!(timestamp);
            let frame = Frame {
                data: data[startaddr..endaddr].to_vec(),
                time: f32::from_le_bytes(data[endaddr .. endaddr+4].try_into().unwrap()),
            };
            // Append frame to CaptureFile 'frames' Vec
            frames.push(frame);

            // Increment startaddr
            startaddr = endaddr + 4;
        }

        CaptureFile {
            height: h, 
            width: w,
            depth: d,
            frames: frames,
        }
    }

    pub fn save_frames_as_png(&self) -> std::io::Result<()> {
        use std::io::Write;
        use std::path::Path;

        let mut filenum = 1;
        let size = self.height * self.width * self.depth;
        for mut frame in &self.frames {
            let mut noalpha = frame.data.clone();
            
            // Remove alpha
            noalpha.iter_mut()
                .enumerate()
                .filter(|(i,_)| i%4 == 3)
                .map(|(_, e)| e)
                .for_each(|x| *x = 255);
            dbg!(&noalpha[0..8]);

            // Swap Red and Blue
            for i in (0..noalpha.len()-2).step_by(4) {
                noalpha.swap(i, i+2);
            }

            // Write Raw Frame
            //let mut file = OpenOptions::new()
            //    .create(true)
            //    .write(true)
            //    .truncate(true)
            //    .open(format!("frame{}", filenum))?;
            //file.write_all(&frame.data[..])?;

            // Write Image
            image::save_buffer(
                &Path::new(format!("output/frame{}.png", filenum).as_str()),
                &noalpha[..],
                self.width,
                self.height,
                image::ColorType::Rgba8)
                .unwrap();

            filenum += 1;
            print!(".");
        }
        Ok(())
    }
}

