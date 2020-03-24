pub mod renderer{
    use crate::ppm_encoder::ppm_encoder::PPM;
    use crate::ppm_encoder::ppm_encoder::RGB;

    pub struct Point{
        pub x: u32,
        pub y: u32
    }
    // Bresenham’s line algorithm
    pub fn draw_line(mut begin: Point, mut end: Point, image: &mut PPM) {
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

        let mut error: i32 = 0;
        let delta_err: i32 = (delta_y + 1);
        let mut y = begin.y;

        let dir_y: i32 = end.y as i32 - begin.y as i32;
        for x in begin.x..end.x+1 {
            if steep{
                image.set_pixel(y, x, &RGB{red: 255, green: 0, blue: 0});
            }
            else {
                image.set_pixel(x, y, &RGB{red: 255, green: 0, blue: 0});
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
}

