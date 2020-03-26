pub mod point {
    pub struct PointFloat {
        pub x: f32,
        pub y: f32,
    }

    pub struct PointInt {
        pub x: i32,
        pub y: i32,
    }

    pub struct Point3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl core::ops::Add for &PointInt {
        type Output = PointInt;

        fn add(self, other: &PointInt) -> PointInt {
            PointInt {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl core::ops::Add for PointInt {
        type Output = PointInt;

        fn add(self, other: PointInt) -> PointInt {
            PointInt {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl core::ops::Sub for PointInt {
        type Output = PointInt;

        fn sub(self, other: PointInt) -> PointInt {
            PointInt {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    impl core::ops::Sub for &PointInt {
        type Output = PointInt;

        fn sub(self, other: &PointInt) -> PointInt {
            PointInt {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    impl core::ops::Mul<PointInt> for PointInt {
        type Output = PointInt;

        fn mul(self, other: PointInt) -> PointInt {
            PointInt {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }
    impl core::ops::Mul<f32> for PointInt {
        type Output = PointInt;
        
        fn mul(self, other: f32) -> PointInt {
            PointInt {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
            }
        }
    }

    impl core::ops::Mul<&PointInt> for &PointInt {
        type Output = PointInt;

        fn mul(self, other: &PointInt) -> PointInt {
            PointInt {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }
    impl core::ops::Mul<f32> for &PointInt {
        type Output = PointInt;
        
        fn mul(self, other: f32) -> PointInt {
            PointInt {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
            }
        }
    }

    impl PointInt {
        pub fn clone(&self) -> PointInt {
            PointInt {
                x: self.x,
                y: self.y,
            }
        }
    }
}

pub mod triangle {
    use crate::geometry::point::{PointInt, PointFloat};

    pub struct TriangleFloat {
        pub p1: PointFloat,
        pub p2: PointFloat,
        pub p3: PointFloat,
    }

    pub struct TriangleInt {
        pub p1: PointInt,
        pub p2: PointInt,
        pub p3: PointInt,
    }

    impl TriangleInt {
        pub fn in_triangle(&self, p: &PointInt) -> bool {
            let tp = p;
            let tp = tp - &self.p1;
    
            let mut b = &self.p2 - &self.p1;
            let mut c = &self.p3 - &self.p1;
    
            if c.y == 0 {
                std::mem::swap(&mut c, &mut b);
            }
    
            let w1 = (tp.y * c.x - tp.x * c.y) as f32 / (b.y * c.x - b.x * c.y) as f32;
            if w1 > 1. || w1 < 0. { return false; }
            let w2 = (tp.y as f32 - w1 * b.y as f32) / (c.y as f32);
    
            return w1 >= 0. && w2 >= 0. && (w1 + w2) <= 1.;
        }
    
        // Faster algo ~25%
        pub fn in_triangle_f(&self, p: &PointInt) -> bool {
            let a_side = (self.p1.y - self.p2.y) * p.x + (self.p2.x - self.p1.x) * p.y + (self.p1.x * self.p2.y - self.p2.x * self.p1.y);
            let b_side = (self.p2.y - self.p3.y) * p.x + (self.p3.x - self.p2.x) * p.y + (self.p2.x * self.p3.y - self.p3.x * self.p2.y);
            let c_side = (self.p3.y - self.p1.y) * p.x + (self.p1.x - self.p3.x) * p.y + (self.p3.x * self.p1.y - self.p1.x * self.p3.y);
            
            return (a_side >= 0 && b_side >= 0 && c_side >= 0) || (a_side <= 0 && b_side <= 0 && c_side <= 0);
        }
    }

}