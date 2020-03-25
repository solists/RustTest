// Encodes in P6 format
pub mod ppm_encoder {
    use std::path::Path;
    use std::io::Write;
    use std::fs::File;

    pub const COLOR_DIMENSIONS: u32 = 3;

    pub struct RGB {
        pub red: u8,
        pub green: u8,
        pub blue: u8
    }

    pub struct PPM {
        pub width: u32,
        pub height: u32,
        pub bits_pp: u8,
        pub data: Vec<u8>
    }

    impl PPM {
        pub fn new(w: u32, h: u32, bpp: u8) -> PPM {
            let size = COLOR_DIMENSIONS * w * h;
            let buffer = vec![0; size as usize];
            PPM { width: w, height: h, bits_pp: bpp, data: buffer }
        }
    }

    impl PPM {
        fn buffer_size(&self) -> u32 {
            3 * self.height * self.width
        }

        fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
            // Origin at the left bottom corner
            let offset = ((self.height - y) * self.width * 3) + (x * 3);
            if offset < self.buffer_size() {
                Some(offset as usize)
            } else {
                None
            }
        }

        pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
            match self.get_offset(x, y) {
                Some(o) => {
                    Some(RGB { red: self.data[o], green: self.data[o + 1], blue: self.data[o + 2] })
                },
                None => None
            }
        }

        pub fn set_pixel(&mut self, x: u32, y: u32, color: &RGB) -> bool {
            if color.red > self.bits_pp ||
                color.green > self.bits_pp ||
                color.blue > self.bits_pp {
                return false;
            }
            match self.get_offset(x, y) {
                Some(o) => {
                    self.data[o] = color.red;
                    self.data[o + 1] = color.green;
                    self.data[o + 2] = color.blue;
                    true
                },
                None => false
            }
        }

        pub fn write_image(&self, filename: &str) -> std::io::Result<()> {
            let path = Path::new(filename);
            let mut file = (File::create(&path))?;
            let header = format!("P6 {} {} {}\n", self.width, self.height, self.bits_pp);
            (file.write_all(header.as_bytes()))?;
            (file.write_all(&self.data))?;
            Ok(())
        }
    }
}