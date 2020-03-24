/*use std::io::{Read, Write};
use std::mem;
use std::slice;
use std::fs;
use std::fs::File;

fn main() {
    //test_inv();
    test_output_file();
    test_input_file();
}

fn test_input_file() {

}

fn test_output_file() {
    let arr: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let mut f = File::create("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.hh").expect("Unable to create file");

    f.write_all(&arr);
}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Configuration {
    item1: u8,
    item2: u16,
    item3: i32,
    item4: [u8; 2]
}


// Prolly packed will be better solution
//#[repr(C)]
//#[repr(packed)]
struct TGAHdr {
    id_length: i8,
    color_map_type: i8,
    data_type_code: i8,
    color_map_origin: i16,
    color_map_length: i16,
    color_map_depth: i8,
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bits_per_pixel: i8,
    image_descriptor: i8
}
enum RGBARepr {
    bgra(u8, u8, u8, u8),
    raw([u8; 4]),
    val(u32)
}

struct TGAColor {
    un: RGBARepr,
    bytes_pp: i32
}

impl TGAColor {
    fn get_default_tga_color() -> TGAColor {
        TGAColor {un: RGBARepr::val(0), bytes_pp: 1}
    }
    fn get_tga_color_with_rgba(R: u8, G: u8, B: u8, A: u8) -> TGAColor {
        TGAColor {un: RGBARepr::bgra(B, G, R, A), bytes_pp: 4}
    }
    fn get_tga_color_with_val(v: u32, b: i32) -> TGAColor {
        TGAColor {un: RGBARepr::val(v), bytes_pp: b}
    }
    fn get_tga_color_from(other: TGAColor) -> TGAColor {
        TGAColor {un: other.un, bytes_pp: other.bytes_pp}
    }
}



struct TGAImage {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_pp: i32,
    //fmt: (u8, u8, u8)
}

impl TGAImage {
    fn get_default_tga_image() -> TGAImage {
        TGAImage {data: [0 as u8].to_vec(), width: 0, height: 0, bytes_pp: 0}
    }
    fn get_tga_image_with(w: i32, h: i32, b: i32) -> TGAImage {
        TGAImage {data: Vec::new(), width: w,
            height: h, bytes_pp: b}
    }
    fn get_tga_image_from(other: TGAImage) -> TGAImage {
        TGAImage {data: other.data, width: other.width,
            height: other.height, bytes_pp: other.bytes_pp}
    }
}

impl TGAImage {
    fn read_tga_file(&mut self, path: &str) {
        if !self.data.is_empty(){
            self.data.clear();
        }

        // Open file

        // Tga hdr
        // read then from file

    }
}*/

mod ppm_encoder;
mod ppm_decoder;
use ppm_encoder::ppm_encoder::PPM;
use ppm_encoder::ppm_encoder::RGB;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    for i in 0..150 {
        let a = PPM::new(1920, 1080, 255);
        a.write_image("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm")
            .expect("Error, while writing an image black");

        let image = match ppm_decoder::ppm_decoder::read_image("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm") {
            (Some(p), _) => {
                Some(p)
            },
            (None, s) => {
                println!("{}", s);
                None
            }
        };

        let mut unpacked_img = image.unwrap();

        // Draw in green
        for x in 0..unpacked_img.width {
            for y in 0..unpacked_img.height {
                unpacked_img.set_pixel(x, y, &RGB { red: 0, green: 0, blue: 100 });
            }
        }

        unpacked_img.write_image("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm")
            .expect("Error, while writing an image green");
    }

    println!("{}", now.elapsed().as_millis());
}
