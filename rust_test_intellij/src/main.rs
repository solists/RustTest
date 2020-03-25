mod ppm_encoder;
mod ppm_decoder;
mod renderer;
mod obj_model;
mod triangle;
mod geometry;
use ppm_encoder::ppm_encoder::PPM;
use ppm_encoder::ppm_encoder::RGB;
use std::time::{Instant};
//use ppm_decoder::ppm_decoder::read_image;
//use renderer::renderer::draw_line;
use geometry::geometry::Point3;
use crate::obj_model::obj_model::Model;
//use crate::geometry::geometry::Point;
//use crate::geometry::geometry::Triangle;
use crate::geometry::geometry::TriangleInt;
use crate::geometry::geometry::PointInt;

fn main() {
    /*let mut img = match read_image("U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm"){
        (Some(t),_) => t,
        (None, s) => {
            println!("{}", s);
            return;
        }
    };*/

    let red = RGB{red: 255, green: 0, blue: 0};
    let green = RGB{red: 0, green: 255, blue: 0};
    let blue = RGB{red: 0, green: 0, blue: 255};
    let white = RGB{red: 255, green: 255, blue: 255};

    let obj_path_linx = "/home/semen/Prog/RustTemp/RustTest/rust_test_intellij/obj/1.obj";
    let obj_path_win = "U:/Users/Semen/Documents/RustTest/rust_test_intellij/obj/1.obj";
    let img_path_linx = "/home/semen/Prog/RustTemp/RustTest/rust_test_intellij/src/temp.ppm";
    let img_path_win = "U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm";

    let n = Instant::now();

    for _i in 0..1 {
        //test_o_max_coord(obj_path, &model);
        let mut image = PPM::new(100, 100, 255);
        //let mut model = Model::new();
        //model.read_obj(obj_path_win);
        //renderer::renderer::print_obj_in_lines(&obj_path_win, &mut image, &RGB{red: 255, green: 0, blue: 0});
        let tr = TriangleInt{
            p1: PointInt{x: 1, y: 2},
            p2: PointInt{x: 3, y: 6},
            p3: PointInt{x: 10, y: 9}
        };
        let a = PointInt{x: 23, y: 15};
        let b = PointInt{x: 18, y: 5};
        let c = &a + &b;
        let in_p = &PointInt{x: 4, y: 5};
        let isin = triangle::triangle::in_triangle(&in_p, &tr);
        //renderer::renderer::draw_triangle(&a, &b, &in_p, &mut image, &white);
        renderer::renderer::draw_triangle_t(&tr, &mut image, &white);
        renderer::renderer::draw_point(&in_p, &mut image, &red);
        println!("{}", isin);
        image.write_image(&img_path_win).expect("Error while writing an image!");
        //println!("{}, {}", c.x, c.y);
    }


    println!("{}", n.elapsed().as_millis());
}


fn test_o_max_coord(obj_path: &str, model: &Model) -> Point3 {
    model.max_coord()
}
