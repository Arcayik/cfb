pub struct Framebuffer {
    device: String,
    width: u16,
    height: u16,
    depth: u8,
}

impl Framebuffer {
    pub fn from_device(dev: &str) -> Framebuffer {
        let sysfolder = format!("/sys/class/graphics/{}/", dev);
        let depth: u8 = std::fs::read_to_string(format!("{sysfolder}/bits_per_pixel")).unwrap()
            .trim().parse().unwrap();
        let dimensions = std::fs::read_to_string(format!("/sys/class/graphics/{}/virtual_size", dev)).unwrap();
        let (width, height) = match dimensions.trim().split_once(',') {
            Some((w,h)) => (w.trim().parse::<u16>().unwrap(),
                            h.trim().parse::<u16>().unwrap()),
            None => panic!("failed to parse virtual_size file"),
        };

        Framebuffer {
            device: dev.to_owned(),
            width: width,
            height: height,
            depth: depth,
        }
    }

    pub fn get_width(&self) -> u16 { self.width }
    pub fn get_height(&self) -> u16 { self.height }
    pub fn get_depth(&self) -> u8 { self.depth }

    pub fn get_mem_size(&self) -> usize {
        usize::from(self.width * self.height * (u16::from(self.depth)/8))
    }

    pub fn get_path(&self) -> String {
        format!("/dev/{}", self.device)
    }
}
