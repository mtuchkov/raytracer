use crate::material::Material;
use crate::vec::{Ray, Vec3};
use crate::surfaces::hitable::{Hitable, HitRecord};
use crate::surfaces::Surface;
use crate::surfaces::Surface::{Sphere};

impl Hitable for Surface {
    fn hit<'a>(&'a self, r: &'a Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Sphere { center, radius, material } =>
                hit_sphere(r, t_min, t_max, center, *radius, material)
        }
    }
}

// LEARN: In the book the hit_sphere accepts the hit_record as a mutable reference and returns bool
// In Rust the idiomatic way is to return an Option<HitRecord> instead.
fn hit_sphere<'a>(r: &'a Ray, t_min: f32, t_max: f32, center: &'a Vec3, radius: f32, material: &'a Material) -> Option<HitRecord<'a>> {

    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
        let mut t = (-b - discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = r.point_at(t);
            let normal = (&p - center) / radius;
            return Some(HitRecord { t, p, normal, material });
        }
        t = (-b + discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = r.point_at(t);
            let normal = (&p - center) / radius;
            return Some(HitRecord { t, p, normal, material });
        }
    }
    None
}