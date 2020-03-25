pub mod geometry {
    pub struct TriangleFloat {
        pub p1: PointFloat,
        pub p2: PointFloat,
        pub p3: PointFloat,
    }

    pub struct PointFloat {
        pub x: f32,
        pub y: f32,
    }

    pub struct PointInt {
        pub x: i32,
        pub y: i32,
    }

    pub struct TriangleInt {
        pub p1: PointInt,
        pub p2: PointInt,
        pub p3: PointInt,
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
    impl core::ops::Mul for PointInt {
        type Output = PointInt;

        fn mul(self, other: PointInt) -> PointInt {
            PointInt {
                x: self.x * other.x,
                y: self.y * other.y,
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