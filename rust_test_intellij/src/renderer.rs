pub mod renderer{
    use crate::ppm_encoder::ppm_encoder::PPM;
    use crate::ppm_encoder::ppm_encoder::RGB;
    //use crate::ppm_encoder;
    use crate::obj_model::obj_model::Model;
    //use crate::geometry::geometry::Point3;
    use crate::geometry::point::{Point2, Point3};
    use crate::geometry::triangle::{Triangle};
    use crate::geometry::vector::Vector3;
    //use crate::geometry::vector;

    // Buffer used to determine which object to render, if they overlapping each other
    pub struct ZBuffer {
        pub data: Vec<i32>
    }

    impl ZBuffer {
        pub fn new(size: i32) -> ZBuffer {
            ZBuffer {
                data: vec![i32::min_value(); size as usize],
            }
        }
    }



    // Bresenham’s line algorithm
    pub fn draw_line(b: &Point3<i32>, e: &Point3<i32>, image: &mut PPM, color: &RGB) {
        let mut begin = b.clone();
        let mut end = e.clone();
        let mut steep = false;
        let delta_x = (begin.x as i32 - end.x as i32).abs();
        let delta_y = (begin.y as i32 - end.y as i32).abs();
        if delta_x < delta_y {
            steep = true;
            std::mem::swap(&mut begin.x, &mut begin.y);
            std::mem::swap(&mut end.x, &mut end.y);
        }
        if begin.x > end.x {
            std::mem::swap(&mut begin, &mut end);
        }

        let delta_x = (begin.x as i32 - end.x as i32).abs();
        let delta_y = (begin.y as i32 - end.y as i32).abs();

        let mut error: i32 = 0;
        let delta_err: i32 = delta_y + 1;
        let mut y = begin.y;

        let dir_y: i32 = end.y as i32 - begin.y as i32;
        for x in begin.x as u32..(end.x as u32+1) {
            if steep{
                image.set_pixel(y as u32, x, color);
            }
            else {
                image.set_pixel(x, y as u32, color);
            }
            error += delta_err;
            if error > (delta_x + 1){
                if dir_y > 0{
                    y += 1;
                }
                else if dir_y < 0{
                    y -= 1;
                }
                error -= delta_x + 1;
            }
        }
    }

    pub fn draw_triangle(p1: &Point3<i32>, p2: &Point3<i32>, p3: &Point3<i32>, image: &mut PPM, color: &RGB) -> bool {
        draw_line(p1, p2, image, color);
        draw_line(p2, p3, image, color);
        draw_line(p3, p1, image, color);

        true
    }

    pub fn draw_triangle_t(tr: &Triangle<i32>, image: &mut PPM, color: &RGB) -> bool {
        draw_line(&tr.p1, &tr.p2, image, color);
        draw_line(&tr.p2, &tr.p3, image, color);
        draw_line(&tr.p3, &tr.p1, image, color);

        true
    }

    pub fn draw_filled_triangle(tr: &Triangle<i32>, image: &mut PPM, color: &RGB) -> bool {
        if tr.p1.x == tr.p2.x && tr.p2.x == tr.p3.x {return false;}
        if tr.p1.y == tr.p2.y && tr.p2.y == tr.p3.y {return false;}

        let x_max = tr.p1.x.max(tr.p2.x).max(tr.p3.x);
        let x_min = tr.p1.x.min(tr.p2.x).min(tr.p3.x);
        let y_max = tr.p1.y.max(tr.p2.y).max(tr.p3.y);
        let y_min = tr.p1.y.min(tr.p2.y).min(tr.p3.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = Point2{x: x, y: y};
                if tr.in_triangle_f(&p) {
                    draw_point(&p, image, color);
                }
            }
        }
        true
    }

    // Twice faster than draw_filled_triangle method
    pub fn draw_filled_triangle_f(tr: &Triangle<i32>, image: &mut PPM, color: &RGB, z_buffer: &mut ZBuffer) -> bool {
        if tr.p1.x == tr.p2.x && tr.p2.x == tr.p3.x {return false;}
        if tr.p1.y == tr.p2.y && tr.p2.y == tr.p3.y {return false;}

        let mut p1 = tr.p1.clone();
        let mut p2 = tr.p2.clone();
        let mut p3 = tr.p3.clone();
        if p1.y > p2.y { std::mem::swap(&mut p1, &mut p2); }
		if p1.y > p3.y { std::mem::swap(&mut p1, &mut p3); }
        if p2.y > p3.y { std::mem::swap(&mut p2, &mut p3); }
        
        let total_height = p3.y as f32 - p1.y as f32;

        for i in 0..total_height as i32 {
            let second_half = i > p2.y - p1.y || p2.y == p1.y;
            let segment_height = if second_half { p3.y as f32 - p2.y as f32 } else { p2.y as f32 - p1.y as f32 };
            let alpha = i as f32 / total_height;
            // Careful: with above conditions no division by zero here
            let temp_coeff = if second_half {p2.y as f32 - p1.y as f32} else {0.};
            let beta = (i as f32 - temp_coeff) / segment_height;
            let mut a = &p1 + &((&p3 - &p1) * alpha);
            let mut b = if second_half {&p2 + &((&p3 - &p2) * beta) } else { &p1 + &((&p2 - &p1) * beta)};
            if a.x > b.x { std::mem::swap(&mut a, &mut b); }
            let af = a.to_float();
            let bf = b.to_float();
			for j in a.x..=b.x {
                // Attention, due to int casts p1.y+i != a.y
                let phi = if b.x == a.x { 1. } else { (j as f32 - &af.x)/(&bf.x - af.x) };
                let p = ((&af + &((&bf - &af)*phi)) ).to_int();
                let idx = p.x + p.y * image.width as i32;
                if z_buffer.data[idx as usize] <= p.z  {
                    z_buffer.data[idx as usize] = p.z;
                    draw_point(&Point2{x: j, y: p1.y + i}, image, color);
                }
			}
        }
        true
    }

    pub fn print_obj_in_lines(model: &Model, image: &mut PPM, color: &RGB) -> bool {
        // Scale object to fit the screen according to the next properties
        let max_p = model.max_coord();
        let offset: f32 = max_p.x.max(max_p.y);
        let scale: f32 = offset * 2.;

        for i in 0..model.faces.len() {
            // Take bended vertices, actually forming one face
            let cur_faces: [u32; 3] = [model.faces[i].f1.v, model.faces[i].f2.v, model.faces[i].f3.v];
            for k in 0..3{
                // Only x, y, dimensions
                let v0 = model.vertices.get(&cur_faces[k]).unwrap();
                let v1 = model.vertices.get(&cur_faces[(k+1)%3]).unwrap();
                let p1 = Point3 {
                    x: ((v0.x + offset)*image.width as f32/scale) as i32,
                    y: ((v0.y + offset)*image.height as f32/scale) as i32,
                    z: v0.z as i32};
                let p2 = Point3 {
                    x: ((v1.x + offset)*image.width as f32/scale) as i32,
                    y: ((v1.y + offset)*image.height as f32/scale) as i32,
                    z: v1.z as i32};
                draw_line(&p1, &p2, image, &color);
            }
        }
        true
    }

    pub fn print_obj_in_triangles(model: &Model, image: &mut PPM, color: &RGB, is_filled: bool, z_buffer: &mut ZBuffer) -> bool {
        // Scale object to fit the screen according to the next properties
        let max_p = model.max_coord();
        let offset: f32 = max_p.x.max(max_p.y);
        let scale: f32 = offset * 2.;
        let width = image.width;
        let height = image.height;

        // Returns vertex world & scaled coordinates
        let get_point = |vertex: u32| {
            let v = model.vertices.get(&vertex).unwrap();
            let p3i = Point3 {
                x: ((v.x + offset)*width as f32/scale) as i32,
                y: ((v.y + offset)*height as f32/scale) as i32,
                // So if it is casted to int, we multiply it to distinguish
                // two different float values further, int due to faster calculations
                z: (v.z * 2.) as i32,
            };
            let p3f = Point3 {
                x: v.x,
                y: v.y,
                z: v.z,
            };

            (p3i, p3f)
        };
        // Returns triangle in world coordinates, and scaled one
        let get_triangles = |i: usize| {
            let (p1scaled, p1world) = get_point(model.faces[i].f1.v);
            let (p2scaled, p2world) = get_point(model.faces[i].f2.v); 
            let (p3scaled, p3world) = get_point(model.faces[i].f3.v);
            let trf = Triangle {
                p1: p1world,
                p2: p2world,
                p3: p3world,
            };
            let tri = Triangle {
                p1: p1scaled,
                p2: p2scaled,
                p3: p3scaled,
            };

            (trf, tri)
        };

        for i in 0..model.faces.len() {
            let (face_in_world, cur_face) = get_triangles(i);
            let light_dir = Vector3{x: 0., y: 0., z: 1.};
            let normal = face_in_world.calc_normal_v();

            let intensity = (normal * light_dir).to_float();

            if intensity <= 0. {continue;}
            
            if is_filled {draw_filled_triangle_f(&cur_face, image, &(color * intensity), z_buffer);}
            else {draw_triangle_t(&cur_face, image, &(color * intensity));}
        }
        true
    }

    pub fn draw_point(point: &Point2<i32>, image: &mut PPM, color: &RGB) {
        image.set_pixel(point.x as u32, point.y as u32, color);
    }

}

