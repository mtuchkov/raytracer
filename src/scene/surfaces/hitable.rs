use crate::scene::material::Material;
use crate::math::vec::{Ray, Vec3};

pub(crate) struct HitRecord<'a> {
    // LEARN:
    // It is completely normal and common to have public fields in Rust
    // Although in some places getters can be useful.
    pub(crate) t: f32,
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: &'a Material
}

pub(crate) trait Hitable {
    /// LEARN:
    /// The original book uses a mutable reference to HitRecord.
    /// This is not idiomatic Rust. We use an Option of HitRecord instead.
    fn hit<'a>(&'a self, r: &'a Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
