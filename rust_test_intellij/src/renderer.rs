pub mod renderer{
    use crate::ppm_encoder::ppm_encoder::PPM;
    use crate::ppm_encoder::ppm_encoder::RGB;
    //use crate::ppm_encoder;
    use crate::obj_model::obj_model::Model;
    //use crate::geometry::geometry::Point3;
    use crate::geometry::point::PointInt;
    use crate::geometry::triangle::TriangleInt;



    // Bresenham’s line algorithm
    pub fn draw_line(b: &PointInt, e: &PointInt, image: &mut PPM, color: &RGB) {
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

    pub fn draw_triangle(p1: &PointInt, p2: &PointInt, p3: &PointInt, image: &mut PPM, color: &RGB) -> bool {
        draw_line(p1, p2, image, color);
        draw_line(p2, p3, image, color);
        draw_line(p3, p1, image, color);

        true
    }

    pub fn draw_triangle_t(tr: &TriangleInt, image: &mut PPM, color: &RGB) -> bool {
        draw_line(&tr.p1, &tr.p2, image, color);
        draw_line(&tr.p2, &tr.p3, image, color);
        draw_line(&tr.p3, &tr.p1, image, color);

        true
    }

    pub fn draw_filled_triangle(tr: &TriangleInt, image: &mut PPM, color: &RGB) -> bool {
        if tr.p1.x == tr.p2.x && tr.p2.x == tr.p3.x {return false;}
        if tr.p1.y == tr.p2.y && tr.p2.y == tr.p3.y {return false;}

        let x_max = tr.p1.x.max(tr.p2.x).max(tr.p3.x);
        let x_min = tr.p1.x.min(tr.p2.x).min(tr.p3.x);
        let y_max = tr.p1.y.max(tr.p2.y).max(tr.p3.y);
        let y_min = tr.p1.y.min(tr.p2.y).min(tr.p3.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = PointInt{x: x, y: y};
                if tr.in_triangle_f(&p) {
                    draw_point(&p, image, color);
                }
            }
        }
        true
    }

    // Twice faster than draw_filled_triangle method
    pub fn draw_filled_triangle_f(tr: &TriangleInt, image: &mut PPM, color: &RGB) -> bool {
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
			for j in a.x..=b.x {
                // Attention, due to int casts p1.y+i != a.y
                draw_point(&PointInt{x: j, y: p1.y + i}, image, color);
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
                let p1 = PointInt {
                    x: ((v0.x + offset)*image.width as f32/scale) as i32,
                    y: ((v0.y + offset)*image.height as f32/scale) as i32};
                let p2 = PointInt {
                    x: ((v1.x + offset)*image.width as f32/scale) as i32,
                    y: ((v1.y + offset)*image.height as f32/scale) as i32};
                draw_line(&p1, &p2, image, &color);
            }
        }
        true
    }

    pub fn print_obj_in_triangles(model: &Model, image: &mut PPM, color: &RGB, is_filled: bool) -> bool {
        // Scale object to fit the screen according to the next properties
        let max_p = model.max_coord();
        let offset: f32 = max_p.x.max(max_p.y);
        let scale: f32 = offset * 2.;
        let width = image.width;
        let height = image.height;

        let get_point = |vertex: u32| {
            let v = model.vertices.get(&vertex).unwrap();
            PointInt {
                x: ((v.x + offset)*width as f32/scale) as i32,
                y: ((v.y + offset)*height as f32/scale) as i32,
            }
        };
        

        for i in 0..model.faces.len() {
            // Take bended vertices, actually forming one face
            let cur_face: TriangleInt = TriangleInt{
                p1: get_point(model.faces[i].f1.v), 
                p2: get_point(model.faces[i].f2.v), 
                p3: get_point(model.faces[i].f3.v),
            };
            let red = RGB{red: 255, green: 0, blue: 0};
            let green = RGB{red: 0, green: 255, blue: 0};
            let blue = RGB{red: 0, green: 0, blue: 255};
            let white = RGB{red: 255, green: 255, blue: 255};
            //if i % 4 == 0       {draw_filled_triangle_f(&cur_face, image, &red);}
            //else if i % 4 == 1  {draw_filled_triangle_f(&cur_face, image, &green);}
            //else if i % 4 == 2  {draw_filled_triangle_f(&cur_face, image, &blue);}
            //else                {draw_filled_triangle_f(&cur_face, image, &white);}
            if is_filled {draw_filled_triangle_f(&cur_face, image, color);}
            else {draw_triangle_t(&cur_face, image, color);}
        }
        true
    }

    pub fn draw_point(point: &PointInt, image: &mut PPM, color: &RGB) {
        image.set_pixel(point.x as u32, point.y as u32, color);
    }

}

