use crate::vec::{Ray, Vec3};

pub(crate) struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            ll_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub(crate) fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from(
            self.origin.clone(),
            &self.ll_corner + u * &self.horizontal + v * &self.vertical - &self.origin)
    }
}