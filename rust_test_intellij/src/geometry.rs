pub mod point {
    use crate::geometry::vector::Vector3;
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
    pub struct Point3Int {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Point3 {
        pub fn toVector3(&self) -> Vector3 {
            Vector3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
    impl Point3Int {
        pub fn toVector3(&self) -> Vector3 {
            Vector3 {
                x: self.x as f32,
                y: self.y as f32,
                z: self.z as f32,
            }
        }
    }

    impl core::ops::Add for &Point3 {
        type Output = Point3;

        fn add(self, other: &Point3) -> Point3 {
            Point3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl core::ops::Sub for &Point3 {
        type Output = Point3;

        fn sub(self, other: &Point3) -> Point3 {
            Point3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
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
        pub fn toVector3(&self) -> Vector3 {
            Vector3 {
                x: self.x as f32,
                y: self.y as f32,
                z: 0.,
            }
        }
    }

    impl core::ops::Add for &Point3Int {
        type Output = Point3Int;

        fn add(self, other: &Point3Int) -> Point3Int {
            Point3Int {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl core::ops::Add for Point3Int {
        type Output = Point3Int;

        fn add(self, other: Point3Int) -> Point3Int {
            Point3Int {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl core::ops::Sub for Point3Int {
        type Output = Point3Int;

        fn sub(self, other: Point3Int) -> Point3Int {
            Point3Int {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
    impl core::ops::Sub for &Point3Int {
        type Output = Point3Int;

        fn sub(self, other: &Point3Int) -> Point3Int {
            Point3Int {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.y - other.z,
            }
        }
    }
    impl core::ops::Mul<Point3Int> for Point3Int {
        type Output = Point3Int;

        fn mul(self, other: Point3Int) -> Point3Int {
            Point3Int {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl core::ops::Mul<f32> for Point3Int {
        type Output = Point3Int;
        
        fn mul(self, other: f32) -> Point3Int {
            Point3Int {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
                z: (self.z as f32 * other) as i32,
            }
        }
    }

    impl core::ops::Mul<&Point3Int> for &Point3Int {
        type Output = Point3Int;

        fn mul(self, other: &Point3Int) -> Point3Int {
            Point3Int {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.y,
            }
        }
    }
    impl core::ops::Mul<f32> for &Point3Int {
        type Output = Point3Int;
        
        fn mul(self, other: f32) -> Point3Int {
            Point3Int {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
                z: (self.z as f32 * other) as i32,
            }
        }
    }

    impl Point3Int {
        pub fn clone(&self) -> Point3Int {
            Point3Int {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
}

pub mod triangle {
    use crate::geometry::point::{PointInt, Point3, Point3Int};
    use crate::geometry::vector::{Vector3};

    pub struct TriangleFloat {
        pub p1: Point3,
        pub p2: Point3,
        pub p3: Point3,
    }
    impl TriangleFloat {
        pub fn calc_normal_v(&self) -> Vector3 {
            let v1 = (&self.p2 - &self.p1).toVector3();
            let v2 = (&self.p3 - &self.p1).toVector3();

            (v1.calc_cross_product(&v2)).normalize()
        }
    }

    pub struct TriangleInt {
        pub p1: Point3Int,
        pub p2: Point3Int,
        pub p3: Point3Int,
    }

    impl TriangleInt {
        pub fn in_triangle(&self, p: &Point3Int) -> bool {
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

        pub fn calc_normal_v(&self) -> Vector3 {
            let v1 = (&self.p2 - &self.p1).toVector3();
            let v2 = (&self.p3 - &self.p1).toVector3();

            (v2.calc_cross_product(&v1)).normalize()
        }

        
    }
}

pub mod vector {
    use crate::inv_sqrt::fast_inv::{inv_f};

    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl Vector3 {
        pub fn normalize(&self) -> Vector3 {
            let coeff = inv_f(self.x*self.x + self.y*self.y + self.z*self.z);
            self * coeff
        }

        pub fn calc_dot_product(&self, other: &Vector3) -> Vector3 {
            self * other
        }

        pub fn calc_cross_product(&self, other: &Vector3) -> Vector3 {
            Vector3 {
                x: self.y * other.z - self.z * other.y, 
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }

        pub fn to_float(&self) -> f32 {
            self.x + self.y + self.z
        }

    }
    impl std::ops::Mul<f32> for &Vector3 {
        type Output = Vector3;
        
        fn mul(self, other: f32) -> Vector3 {
            Vector3{
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }
    impl std::ops::Mul<f32> for Vector3 {
        type Output = Vector3;
        
        fn mul(self, other: f32) -> Vector3 {
            Vector3{
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }
    impl std::ops::Mul<&Vector3> for &Vector3 {
        type Output = Vector3;
        
        fn mul(self, other: &Vector3) -> Vector3 {
            Vector3{
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl std::ops::Mul<Vector3> for Vector3 {
        type Output = Vector3;
        
        fn mul(self, other: Vector3) -> Vector3 {
            Vector3{
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
}