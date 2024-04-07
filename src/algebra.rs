use std::ops::Index;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

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

    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Scalar product of two vectors.
    pub(crate) fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Cross product of two vectors.
    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    /// Returns a new vector that is the unit vector of the given vector.
    fn make_unit(v: &Vec3) -> Vec3 {
        v / v.length()
    }

    pub(crate) fn unit(&self) -> Vec3 {
        self / self.length()
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
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// !!! NOTE !!!
/// This implementation of Add consumes both operands and returns a new Vec3 instance.
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Vec3> for &mut Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
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

impl DivAssign<f32> for &mut Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}