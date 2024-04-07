pub(crate) mod hitable;
pub(crate) mod sphere;
pub(crate) mod world;

use crate::surfaces::Surface::Sphere;
use crate::vec::Vec3;

pub(crate) enum Surface {
    Sphere {
        center: Vec3,
        radius: f32,
    }
}

impl Surface {
    pub(crate) fn sphere(center: Vec3, radius: f32) -> Surface {
        Sphere { center, radius }
    }
}