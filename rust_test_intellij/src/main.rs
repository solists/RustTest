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
mod renderer;
mod obj_model;
mod triangle;
mod geometry;
use ppm_encoder::ppm_encoder::PPM;
use ppm_encoder::ppm_encoder::RGB;
use std::time::{Duration, Instant};
use ppm_decoder::ppm_decoder::read_image;
use renderer::renderer::draw_line;
use geometry::geometry::Point3;
use crate::obj_model::obj_model::Model;
use crate::geometry::geometry::Point;
use crate::geometry::geometry::Triangle;

fn main() {
    /*let mut img = match read_image("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm"){
        (Some(t),_) => t,
        (None, s) => {
            println!("{}", s);
            return;
        }
    };*/

    let obj_path = "/home/semen/Prog/RustTemp/RustTest/1.obj";
    let imj_path = "/home/semen/Prog/RustTemp/RustTest/temp.ppm";

    let n = Instant::now();

    for i in 0..1 {
        //test_o_max_coord(obj_path, &model);
        //let mut model = Model::new();
        //model.read_obj(obj_path);
        //renderer::renderer::print_obj_in_lines(&obj_path, &imj_path);
        let tr = Triangle{
            p1: Point{x: 1, y: 2},
            p2: Point{x: 3, y: 6},
            p3: Point{x: 10, y: 9}
        };
        let a = Point{x: 23, y: 15};
        let b = Point{x: 18, y: 5};
        let c = &a + &b;
        let isin = triangle::triangle::in_triangle(&Point{x: 1, y: 4}, &tr);
        println!("{}", isin);
        //println!("{}, {}", c.x, c.y);
    }


    println!("{}", n.elapsed().as_millis());
}


fn test_o_max_coord(obj_path: &str, model: &Model) -> Point3 {
    model.max_coord()
}
