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

    pub fn print_obj_in_lines(obj_path: &str, image: &mut PPM, color: &RGB) -> bool {
        let mut model = Model::new();
        model.read_obj(obj_path);

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

    pub fn draw_point(point: &PointInt, image: &mut PPM, color: &RGB) {
        image.set_pixel(point.x as u32, point.y as u32, color);
    }

}

