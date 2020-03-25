pub mod triangle {
    use crate::geometry::geometry::{Point, Triangle};

    pub fn in_triangle(p: &Point, tri: &Triangle) -> bool {
        let mut tp = p;
        tp = &(tp - &tri.p1);

        let mut b = &tri.p2 - &tri.p1;
        let mut c = &tri.p3 - &tri.p1;

        if c.y == 0 {
            std::mem::swap(&mut c, &mut b);
        }

        let w1 = (p.y * c.x - p.x * c.y) as f32 / (b.y * c.x - b.x * c.y) as f32;
        if w1 > 1. || w1 < 0. { return false; }
        let w2 = (p.y as f32 - w1 * b.y as f32) / (c.y as f32);

        return w1 >= 0. && w2 >= 0. && (w1 + w2) <= 1.;
    }
}