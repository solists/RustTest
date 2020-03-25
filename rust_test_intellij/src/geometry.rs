pub mod geometry {
    pub struct Triangle {
        pub p1: Point,
        pub p2: Point,
        pub p3: Point,
    }

    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    pub struct Point3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl core::ops::Add for &Point {
        type Output = Point;

        fn add(self, other: Self) -> Point {
            Point {
                x: &self.x + &other.x,
                y: &self.y + &other.y,
            }
        }
    }

    impl core::ops::Sub for &Point {
        type Output = Point;

        fn sub(self, other: &Point) -> Point {
            Point {
                x: &self.x - &other.x,
                y: &self.y - &other.y,
            }
        }
    }
    impl core::ops::Mul for Point {
        type Output = Point;

        fn mul(self, other: Point) -> Point {
            Point {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

}