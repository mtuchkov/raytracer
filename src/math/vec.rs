use crate::math::rand::drand32;

/// You almost always want to operate with vectors using algebraic expressions.
///
/// This module contains the structs that required operate with 3D models
/// and implement traits for the basic algebraic operations over them.
/// The main structs are Vec3 and Ray.

/// This struct represents a 3D vector. 3D vectors are used to represent
/// points, directions, offsets, and even colors in the RGB space.
#[derive(Clone, Debug)]
pub(crate) struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/// This struct represents the Ray. It is used to compute the color seen along
/// the path of the light ray as if that would happen in the real world.
/// Ray has a starting point (origin) and a direction.
/// Where the starting point is the eye of the observer or a camera.
pub(crate) struct Ray {
    origin: Vec3,

    /// For purists this should be a unit vector, but for our purposes
    /// it is enough to have any vector that points in the right direction.
    direction: Vec3,
}

impl Vec3 {

    pub(crate) fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub(crate) fn x(&self) -> f32 {
        self.x
    }

    pub(crate) fn y(&self) -> f32 {
        self.y
    }

    pub(crate) fn z(&self) -> f32 {
        self.z
    }

    pub(crate) fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub(crate) fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Scalar product of two vectors.
    pub(crate) fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Cross product of two vectors.
    pub(crate) fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub(crate) fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub(crate) fn basis() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub(crate) fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub(crate) fn rand() -> Vec3 {
        Vec3::new(drand32(), drand32(), drand32())
    }

    pub(crate) fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = 2.0 * Vec3::rand() - Vec3::basis();
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub(crate) fn random_in_unit_disk() -> Vec3 {
        loop {
            let rand_2d = Vec3::new(drand32(), drand32(), 0.);
            let basis_2d = Vec3::new(1.0, 1.0, 0.0);
            let p = 2.0 * rand_2d - basis_2d;
            if Vec3::dot(&p, &p) < 1. {
                return p;
            }
        }
    }
}

impl Ray {
    pub(crate) fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Returns the point at the given distance along the ray.
    pub(crate) fn point_at(&self, t: f32) -> Vec3 {
        &self.origin + &(t * &self.direction)
    }
}

mod algebra {
    use std::ops::{Index, Neg};
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Sub;
    use std::ops::SubAssign;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Div;
    use std::ops::DivAssign;
    use super::Vec3;


    impl Index<usize> for Vec3 {
        type Output = f32;

        fn index(&self, index: usize) -> &f32 {
            assert!(index < 3);
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                _ => unreachable!("Index out of bounds."),
            }
        }
    }

    impl Add<&Vec3> for &Vec3 {
        type Output = Vec3;

        fn add(self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Add<Vec3> for &Vec3 {
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3 {
            self + &other
        }
    }

    impl Add<&Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, other: &Vec3) -> Vec3 {
            &self + other
        }
    }

    impl Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3 {
            &self + &other
        }
    }

    impl AddAssign<&Vec3> for &mut Vec3 {
        fn add_assign(&mut self, other: &Vec3) {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
        }
    }

    impl AddAssign<Vec3> for Vec3 {
        fn add_assign(&mut self, other: Vec3) {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
        }
    }

    impl Sub<&Vec3> for &Vec3 {
        type Output = Vec3;

        fn sub(self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Sub<&Vec3> for Vec3 {
        type Output = Vec3;

        fn sub(self, other: &Vec3) -> Vec3 {
            &self - other
        }
    }

    impl Sub<Vec3> for &Vec3 {
        type Output = Vec3;

        fn sub(self, other: Vec3) -> Vec3 {
            self - &other
        }
    }

    impl Sub<Vec3> for Vec3 {
        type Output = Vec3;

        fn sub(self, other: Vec3) -> Vec3 {
            &self - &other
        }
    }

    impl SubAssign<&Vec3> for &mut Vec3 {
        fn sub_assign(&mut self, other: &Vec3) {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
        }
    }

    impl Mul<&Vec3> for f32 {
        type Output = Vec3;

        fn mul(self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self * other.x,
                y: self * other.y,
                z: self * other.z,
            }
        }
    }

    impl Mul<&Vec3> for &Vec3 {
        type Output = Vec3;

        fn mul(self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }

    impl Mul<Vec3> for &Vec3 {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            self * &other
        }
    }

    impl Mul<Vec3> for f32 {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            self * &other
        }
    }

    impl MulAssign<&Vec3> for &mut Vec3 {
        fn mul_assign(&mut self, other: &Vec3) {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
        }
    }

    impl Mul<f32> for &Vec3 {
        type Output = Vec3;

        fn mul(self, other: f32) -> Vec3 {
            Vec3 {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }

    impl MulAssign<f32> for &mut Vec3 {
        fn mul_assign(&mut self, other: f32) {
            self.x *= other;
            self.y *= other;
            self.z *= other;
        }
    }

    impl Div<&Vec3> for &Vec3 {
        type Output = Vec3;

        fn div(self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
    }

    impl DivAssign<&Vec3> for &mut Vec3 {
        fn div_assign(&mut self, other: &Vec3) {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
        }
    }

    impl Div<f32> for &Vec3 {
        type Output = Vec3;

        fn div(self, other: f32) -> Vec3 {
            Vec3 {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }
    }

    impl Div<f32> for Vec3 {
        type Output = Vec3;

        fn div(self, other: f32) -> Vec3 {
            &self / other
        }
    }

    impl DivAssign<f32> for &mut Vec3 {
        fn div_assign(&mut self, other: f32) {
            self.x /= other;
            self.y /= other;
            self.z /= other;
        }
    }

    impl DivAssign<f32> for Vec3 {
        fn div_assign(&mut self, other: f32) {
            self.x /= other;
            self.y /= other;
            self.z /= other;
        }
    }

    impl Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Vec3 {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Neg for &Vec3 {
        type Output = Vec3;

        fn neg(self) -> Vec3 {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
}