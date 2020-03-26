mod ppm_encoder;
mod ppm_decoder;
mod renderer;
mod obj_model;
mod geometry;
use ppm_encoder::ppm_encoder::PPM;
use ppm_encoder::ppm_encoder::RGB;
use std::time::{Instant};
//use ppm_decoder::ppm_decoder::read_image;
//use renderer::renderer::draw_line;
use geometry::point::Point3;
use crate::obj_model::obj_model::Model;
//use crate::geometry::geometry::Point;
//use crate::geometry::geometry::Triangle;
use crate::geometry::triangle::TriangleInt;
use crate::geometry::point::PointInt;

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
    let obj_path_win = "U:/Users/Semen/Documents/RustTest/rust_test_intellij/obj/2.obj";
    let img_path_linx = "/home/semen/Prog/RustTemp/RustTest/rust_test_intellij/src/temp.ppm";
    let img_path_win = "U:/Users/Semen/Documents/RustTest/rust_test_intellij/src/temp.ppm";

    

    let mut c = 0;
    let mut image = PPM::new(1000, 1000, 255);

    let mut model = Model::new();
    model.read_obj(obj_path_win);

    /*let tri = TriangleInt{
        p1: PointInt{x: 10, y: 20},
        p2: PointInt{x: 30, y: 70},
        p3: PointInt{x: 50, y: 20}
    };*/
    //renderer::renderer::draw_triangle_t(&tri, &mut image, &white);
    //renderer::renderer::draw_triangle(&a, &b, &in_p, &mut image, &white);
    let n = Instant::now();
    for _i in 0..1000 {
        //test_o_max_coord(obj_path, &model);
        
        renderer::renderer::print_obj_in_triangles(&model, &mut image, &red, true);
        //renderer::renderer::print_obj_in_lines(&model, &mut image, &red);
        //let tr = TriangleInt{
        //    p1: PointInt{x: 0, y: 1},
        //    p2: PointInt{x: 3, y: 7},
        //    p3: PointInt{x: 6, y: 1}
        //};
        ////let a = PointInt{x: 23, y: 15};
        ////let b = PointInt{x: 18, y: 5};
        ////let c = &a + &b;
        //let in_p = PointInt{x: _i, y: 1};
        //let isin = tr.in_triangle_f(&in_p);
        //if isin {c += 1;}
        
        //renderer::renderer::draw_point(&PointInt{x: in_p.x * 10, y: in_p.y * 10}, &mut image, &red);
        //println!("{}", isin);
        //
        //println!("{}, {}", c.x, c.y);
    }
    println!("Time in ms: {}", n.elapsed().as_millis());

    image.write_image(&img_path_win).expect("Error while writing an image!");

    println!("np: {}", c);
    
}


fn test_o_max_coord(obj_path: &str, model: &Model) -> Point3 {
    model.max_coord()
}
