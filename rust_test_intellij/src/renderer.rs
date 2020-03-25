pub mod renderer{
    use crate::ppm_encoder::ppm_encoder::PPM;
    use crate::ppm_encoder::ppm_encoder::RGB;
    use crate::ppm_encoder;
    use crate::obj_model::obj_model::Model;
    use crate::geometry::geometry::Point3;
    use crate::geometry::geometry::Point;



    // Bresenham’s line algorithm
    pub fn draw_line(mut begin: Point, mut end: Point, image: &mut PPM, color: &RGB) {
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
        for x in begin.x..end.x+1 {
            if steep{
                image.set_pixel(y, x, color);
            }
            else {
                image.set_pixel(x, y, color);
            }
            error += delta_err;
            if error > (delta_x + 1){
                if dir_y > 0{
                    y += 1;
                }
                else if dir_y < 0{
                    y -= 1;
                }
                error -= (delta_x + 1);
            }
        }
    }

    pub fn draw_triangle(p1: Point, p2: Point, p3: Point, image: &mut PPM) {

    }

    pub fn print_obj_in_lines(obj_path: &str, img_path: &str) {
        let mut img = ppm_encoder::ppm_encoder::PPM::new(1000, 1000, 255);

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
                let p1 = Point {
                    x: ((v0.x + offset)*img.width as f32/scale) as u32,
                    y: ((v0.y + offset)*img.height as f32/scale) as u32};
                let p2 = Point {
                    x: ((v1.x + offset)*img.width as f32/scale) as u32,
                    y: ((v1.y + offset)*img.height as f32/scale) as u32};
                draw_line(p1, p2, &mut img, &RGB{red: 255, green: 0, blue: 0});
            }
        }

        img.write_image(img_path).expect("Error while writing image");
    }

}

