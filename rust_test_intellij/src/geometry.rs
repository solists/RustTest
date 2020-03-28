pub mod point {
    use crate::geometry::vector::Vector3;
    use core::ops::Add;
    use core::ops::Sub;
    use core::ops::Mul;

    pub struct Point2<T> {
        pub x: T,
        pub y: T,
    }
    #[derive(Debug)]
    pub struct Point3<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }



    // Point2 operators****************************
    impl<T> Add for &Point2<T>
    where T: Add<Output=T> + Copy {
        type Output = Point2<T>;

        fn add(self, other: &Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    impl<T> Add for Point2<T>
    where T: Add<Output=T> + Copy {
        type Output = Point2<T>;

        fn add(self, other: Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    impl<T> Sub for Point2<T> 
    where T: Sub<Output=T> + Copy {
        type Output = Point2<T>;

        fn sub(self, other: Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    impl<T> Sub for &Point2<T> 
    where T: Sub<Output=T> + Copy {
        type Output = Point2<T>;

        fn sub(self, other: &Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    impl<T> Mul for Point2<T> 
    where T: Mul<Output=T> + Copy {
        type Output = Point2<T>;

        fn mul(self, other: Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }
    impl<T> Mul for &Point2<T> 
    where T: Mul<Output=T> + Copy {
        type Output = Point2<T>;

        fn mul(self, other: &Point2<T>) -> Point2<T> {
            Point2 {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }
    impl Mul<f32> for &Point2<i32> 
    {
        type Output = Point2<i32>;

        fn mul(self, other: f32) -> Point2<i32> {
            Point2 {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
            }
        }
    }
    impl Mul<f32> for Point2<i32> 
    {
        type Output = Point2<i32>;

        fn mul(self, other: f32) -> Point2<i32> {
            Point2 {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
            }
        }
    }
    impl Mul<f32> for &Point2<f32> 
    {
        type Output = Point2<f32>;

        fn mul(self, other: f32) -> Point2<f32> {
            Point2 {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }
    impl Mul<f32> for Point2<f32> 
    {
        type Output = Point2<f32>;

        fn mul(self, other: f32) -> Point2<f32> {
            Point2 {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }
    impl<T> Point2<T> 
    where T: Copy {
        pub fn clone(&self) -> Point2<T> {
            Point2 {
                x: self.x,
                y: self.y,
            }
        }
    }
    // Point3 operators********************
    impl Point3<i32> {
        pub fn to_vector3(&self) -> Vector3 {
            Vector3 {
                x: self.x as f32,
                y: self.y as f32,
                z: self.z as f32,
            }
        }
        pub fn to_float(&self) -> Point3<f32> {
            Point3 {
                x: self.x as f32,
                y: self.y as f32,
                z: self.z as f32,
            }
        }
    }
    impl Point3<f32> {
        pub fn to_vector3(&self) -> Vector3 {
            Vector3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
        pub fn to_int(&self) -> Point3<i32> {
            Point3 {
                x: (self.x + 0.5) as i32,
                y: (self.y + 0.5) as i32,
                z: (self.z + 0.5) as i32,
            }
        }
    }
    impl<T> core::ops::Add for Point3<T>
    where T: Add<Output=T> + Copy {
        type Output = Point3<T>;

        fn add(self, other: Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl<T> core::ops::Add for &Point3<T>
    where T: Add<Output=T> + Copy {
        type Output = Point3<T>;

        fn add(self, other: &Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl<T> core::ops::Sub for Point3<T>
    where T: Sub<Output=T> + Copy {
        type Output = Point3<T>;

        fn sub(self, other: Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
    impl<T> core::ops::Sub for &Point3<T>
    where T: Sub<Output=T> + Copy {
        type Output = Point3<T>;

        fn sub(self, other: &Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
    impl<T> core::ops::Mul for Point3<T>
    where T: Mul<Output=T> + Copy {
        type Output = Point3<T>;

        fn mul(self, other: Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl<T> core::ops::Mul for &Point3<T>
    where T: Mul<Output=T> + Copy {
        type Output = Point3<T>;

        fn mul(self, other: &Point3<T>) -> Point3<T> {
            Point3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl core::ops::Mul<f32> for &Point3<i32> {
        type Output = Point3<i32>;

        fn mul(self, other: f32) -> Point3<i32> {
            Point3 {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
                z: (self.z as f32 * other) as i32,
            }
        }
    }
    impl core::ops::Mul<f32> for Point3<i32> {
        type Output = Point3<i32>;

        fn mul(self, other: f32) -> Point3<i32> {
            Point3 {
                x: (self.x as f32 * other) as i32,
                y: (self.y as f32 * other) as i32,
                z: (self.z as f32 * other) as i32,
            }
        }
    }
    impl core::ops::Mul<f32> for &Point3<f32> {
        type Output = Point3<f32>;

        fn mul(self, other: f32) -> Point3<f32> {
            Point3 {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }
    impl core::ops::Mul<f32> for Point3<f32> {
        type Output = Point3<f32>;

        fn mul(self, other: f32) -> Point3<f32> {
            Point3 {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }

    impl<T> Point3<T> 
    where T: Copy {
        pub fn clone(&self) -> Point3<T> {
            Point3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
}

pub mod triangle {
    use crate::geometry::point::{Point2, Point3};
    use crate::geometry::vector::{Vector3};
    //use crate::geometry::triangle::edge_fun;

    //type PointInt = Point2<i32>;

    #[derive(Debug)]
    pub struct Triangle<T> {
        pub p1: Point3<T>,
        pub p2: Point3<T>,
        pub p3: Point3<T>,
    }

    impl Triangle<f32> {
        pub fn calc_normal_v(&self) -> Vector3 {
            let v1 = (&self.p2 - &self.p1).to_vector3();
            let v2 = (&self.p3 - &self.p1).to_vector3();

            (v1.calc_cross_product(&v2)).normalize()
        }
    }

    impl Triangle<i32> {
        pub fn in_triangle(&self, p: &Point3<i32>) -> bool {
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

        /*pub fn edge_fun(a: &Point3<i32>, b: &Point3<i32>, c: &Point3<i32>) -> i32 {
            (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
        } */

        pub fn get_barocentryc(&self, p: &Point3<i32>) -> Option<Point3<f32>> {
            let edge_fun = |a: &Point3<i32>, b: &Point3<i32>, c: &Point3<i32>| {
                (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
            };

            let area = edge_fun(&self.p1, &self.p2, &self.p3) as f32; // area of the triangle multiplied by 2 
            let mut w0 = edge_fun(&self.p2, &self.p3, p) as f32; // signed area of the triangle v1v2p multiplied by 2 
            let mut w1 = edge_fun(&self.p3, &self.p1, p) as f32; // signed area of the triangle v2v0p multiplied by 2 
            let mut w2 = edge_fun(&self.p1, &self.p2, p) as f32; // signed area of the triangle v0v1p multiplied by 2 

           
            // if point p is inside triangles defined by vertices v0, v1, v2
            //if w0 >= 0. && w1 >= 0. && w2 >= 0. { 
                w0 /= area;
                w1 /= area;
                w2 /= area;
            // barycentric coordinates are the areas of the sub-triangles divided by the area of the main triangle
                let t = Point3{
                    x: w0,
                    y: w1,
                    z: w2,
                };
                return Some(t);
           // }

            //None
        }
    
        // Faster algo ~25, edge functions
        pub fn in_triangle_f(&self, p: &Point2<i32>) -> bool {
            let a_side = (self.p1.y - self.p2.y) * p.x + (self.p2.x - self.p1.x) * p.y + (self.p1.x * self.p2.y - self.p2.x * self.p1.y);
            let b_side = (self.p2.y - self.p3.y) * p.x + (self.p3.x - self.p2.x) * p.y + (self.p2.x * self.p3.y - self.p3.x * self.p2.y);
            let c_side = (self.p3.y - self.p1.y) * p.x + (self.p1.x - self.p3.x) * p.y + (self.p3.x * self.p1.y - self.p1.x * self.p3.y);
            
            return (a_side >= 0 && b_side >= 0 && c_side >= 0) || (a_side <= 0 && b_side <= 0 && c_side <= 0);
        }

        pub fn calc_normal_v(&self) -> Vector3 {
            let v1 = (&self.p2 - &self.p1).to_vector3();
            let v2 = (&self.p3 - &self.p1).to_vector3();

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
    
    impl std::ops::Add<Vector3> for Vector3 {
        type Output = Vector3;
        
        fn add(self, other: Vector3) -> Vector3 {
            Vector3{
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl std::ops::Add<&Vector3> for Vector3 {
        type Output = Vector3;
        
        fn add(self, other: &Vector3) -> Vector3 {
            Vector3{
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl std::ops::Add<&Vector3> for &Vector3 {
        type Output = Vector3;
        
        fn add(self, other: &Vector3) -> Vector3 {
            Vector3{
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl std::ops::Add<Vector3> for &Vector3 {
        type Output = Vector3;
        
        fn add(self, other: Vector3) -> Vector3 {
            Vector3{
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
}