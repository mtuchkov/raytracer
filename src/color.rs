use crate::vec::{Vec3};

/// LEARN
/// This trait defines the color trait.
///
/// It extends the Vec3 struct with methods related to colors but only available for Vec3 instances
/// when color module is explicitly referenced.
///
/// We could append all of these methods directly to the Vec3, but this way
/// we can keep the Vec3 struct clean and demonstrate the interface composition.
pub(crate) trait Color {
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;

    fn rgb(r:f32, g:f32, b:f32) -> Vec3 {
        Vec3::new(r, g, b)
    }
}

impl Color for Vec3 {
    fn r(&self) -> f32 {
        self.x()
    }

    fn g(&self) -> f32 {
        self.y()
    }

    fn b(&self) -> f32 {
        self.z()
    }
}