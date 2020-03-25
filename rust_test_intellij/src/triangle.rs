pub mod triangle {
    use crate::geometry::geometry::{PointInt, TriangleInt};

    pub fn in_triangle(p: &PointInt, tri: &TriangleInt) -> bool {
        let tp = p;
        let tp = tp - &tri.p1;

        let mut b = &tri.p2 - &tri.p1;
        let mut c = &tri.p3 - &tri.p1;

        if c.y == 0 {
            std::mem::swap(&mut c, &mut b);
        }

        let w1 = (tp.y * c.x - tp.x * c.y) as f32 / (b.y * c.x - b.x * c.y) as f32;
        if w1 > 1. || w1 < 0. { return false; }
        let w2 = (tp.y as f32 - w1 * b.y as f32) / (c.y as f32);

        return w1 >= 0. && w2 >= 0. && (w1 + w2) <= 1.;
    }

    // Faster algo ~25%, does not include edges of triangle
    pub fn in_triangle_f(p: &PointInt, tri: &TriangleInt) -> bool {
        let a_side = (tri.p1.y - tri.p2.y) * p.x + (tri.p2.x - tri.p1.x) * p.y + (tri.p1.x * tri.p2.y - tri.p2.x * tri.p1.y);
		let b_side = (tri.p2.y - tri.p3.y) * p.x + (tri.p3.x - tri.p2.x) * p.y + (tri.p2.x * tri.p3.y - tri.p3.x * tri.p2.y);
        let c_side = (tri.p3.y - tri.p1.y) * p.x + (tri.p1.x - tri.p3.x) * p.y + (tri.p3.x * tri.p1.y - tri.p1.x * tri.p3.y);
        
        return (a_side >= 0 && b_side >= 0 && c_side >= 0) || (a_side < 0 && b_side < 0 && c_side < 0);
    }

}