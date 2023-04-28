use std::fs::OpenOptions;
use std::io::Read;

pub enum FrameFormats {
    Raw,
    Png,
}

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

        // Get Frame Size
        let framesize: usize = (h * w * d) as usize;
        
        // Read Frames
        let mut startaddr: usize = 12;
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            let endaddr = startaddr + framesize;
            if endaddr > data.len() { break }
            // get Frame
            let timestamp = f32::from_le_bytes(data[endaddr .. endaddr+4].try_into().unwrap());
           
            // get data and remove alpha channel
            let mut framedata = data[startaddr..endaddr].to_vec()
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i % 4 != 3)
                .map(|(_, e)| e)
                .collect::<Vec<_>>();

                // Swap Red and Blue
                for i in (0..framedata.len()-2).step_by(3) {
                    framedata.swap(i, i+2);
                }

            let mut frame = Frame {
                data: framedata,
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

    pub fn output_frame(&self, format: FrameFormats) -> std::io::Result<()> {
        use std::io::Write;
        use std::path::Path;

        let mut filenum = 1;
        let size = self.height * self.width * self.depth;
        for mut frame in &self.frames {
            let mut noalpha = frame.data.clone();

            match format {
                FrameFormats::Raw => {
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(format!("frame{}", filenum))?;
                    file.write_all(&frame.data[..])?;
                }
                FrameFormats::Png => {
                    image::save_buffer(
                        &Path::new(format!("output/frame{}.png", filenum).as_str()),
                        &noalpha[..],
                        self.width,
                        self.height,
                        image::ColorType::Rgba8
                    ).unwrap();
                }
            }

            filenum += 1;
            print!(".");
        }
        Ok(())
    }

    pub fn output_video(&mut self) {
        use std::io::{Cursor, Seek, SeekFrom};
        use image::{EncodableLayout, Rgb, RgbImage};
        use minimp4::Mp4Muxer;
        use openh264::encoder::{Encoder, EncoderConfig};

        println!("Compiling...");

        let h = self.height as usize;
        let w = self.width as usize;

        let config = EncoderConfig::new(w.try_into().unwrap(), h.try_into().unwrap());
        let mut encoder = Encoder::with_config(config).unwrap();

        let mut buf = Vec::new();

        for frame in &self.frames {

            // Convert RGB into YUV
            let mut yuv = openh264::formats::YUVBuffer::new(w, h);
            
            // Calculate what fraction of a second the frame takes
            let repeatnum = (frame.time * 60.0).round() as i32;
            println!("FRAME TIME: {}", repeatnum); // 60 fps

            // Write frame repeatnum times to fill 60 fps for proper timing
            for _ in 0..repeatnum {
                yuv.read_rgb(&frame.data[..]);

                // Encode YUV into H.264
                let bitstream = encoder.encode(&yuv).unwrap();

                bitstream.write_vec(&mut buf);
            }
        }

        let mut video_buffer = Cursor::new(Vec::new());
        let mut mp4muxer = Mp4Muxer::new(&mut video_buffer);
        mp4muxer.init_video(w.try_into().unwrap(), h.try_into().unwrap(), false, "Screen Recording");
        mp4muxer.write_video(&buf);
        mp4muxer.close();

        // Get raw bytes for the video.
        video_buffer.seek(SeekFrom::Start(0)).unwrap();
        let mut video_bytes = Vec::new();
        video_buffer.read_to_end(&mut video_bytes).unwrap();

        std::fs::write("recording.mp4", &video_bytes).unwrap();
    }
}

