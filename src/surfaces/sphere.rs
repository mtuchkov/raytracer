use crate::vec::{Ray, Vec3};
use crate::surfaces::hitable::{Hitable, HitRecord};
use crate::surfaces::Surface;
use crate::surfaces::Surface::{Sphere};

impl Hitable for Surface {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Sphere { center, radius } =>
                hit_sphere(r, t_min, t_max, center, *radius)
        }
    }
}

fn hit_sphere(r: &Ray, t_min: f32, t_max: f32, center: &Vec3, radius: f32) -> Option<HitRecord> {

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
            return Some(HitRecord { t, p, normal, });
        }
        t = (-b + discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = r.point_at(t);
            let normal = (&p - center) / radius;
            return Some(HitRecord { t, p, normal, });
        }
    }
    None
}