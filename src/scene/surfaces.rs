pub(crate) mod hitable;
pub(crate) mod sphere;
pub(crate) mod world;

use crate::math::vec::Vec3;
use crate::scene::material::Material;
use crate::scene::surfaces::Surface::Sphere;

pub(crate) enum Surface {
    Sphere {
        center: Vec3,
        radius: f32,
        material: Material,
    }
}

impl Surface {
    pub(crate) fn sphere(center: Vec3, radius: f32, material: Material) -> Surface {
        Sphere { center, radius, material }
    }
}