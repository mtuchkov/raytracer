use crate::surfaces::hitable::HitRecord;
use crate::vec::{Ray, Vec3};

pub(crate) enum Material {
    // randomly diffuses the light
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f32,
    }
}

/// LEARN:
/// In C++ the scatter function is an abstract method of the base class `Material`.
/// In Rust we use traits for that.
///
/// Different materials reflect or refract the light in different ways.
/// This trait will implement the scatter function for each material.
pub(crate) trait Scatterable {
    fn scatter(&self,
               r_in: &Ray,
               rec: HitRecord) -> Option<(Ray, &Vec3)>;
}

impl Material {
    pub(crate) fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian { albedo }
    }
    pub(crate) fn metal(albedo: Vec3, fuzz: f32) -> Material {
        Material::Metal { albedo, fuzz }
    }
}

impl Scatterable for Material {


    // LEARN:
    // In the book the scatter accepts the hit_record as a mutable reference and returns bool
    // In Rust the idiomatic way is to return an Option<(ray: Ray, attenuation:Vec3)> instead.
    // Note that the HitRecord is consumed by this function.
    fn scatter(&self,
                          r_in: &Ray,
                          rec: HitRecord) -> Option<(Ray, &Vec3)> {

        // LEARN:
        // The `match` must be exhaustive. We need to handle all variants of the enum.
        // When we added the `Material::Metal` you could notice that the `Metal` variant is missing
        // and compiler will show an error.
        // The analog of the `default` case in C++ or Java is the `_` in Rust.

        match self {
            // LEARN:
            // the enum is destructed her and the structs fields are accessed by ref.
            Material::Lambertian { albedo } => {
                let target = &rec.p + &rec.normal + Vec3::random_in_unit_sphere();
                let direction = target - &rec.p;
                let scattered = Ray::from(rec.p, direction);
                let attenuation = albedo;
                Some((scattered, attenuation))
            }
            Material::Metal { albedo, fuzz } => {
                fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
                    v - &(2.0 * Vec3::dot(v, n) * n)
                }

                let reflected = reflect(&r_in.direction().unit(), &rec.normal);
                let direction = reflected + *fuzz * Vec3::random_in_unit_sphere();
                let scattered = Ray::from(rec.p, direction);
                let attenuation = albedo;
                if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
                    Some((scattered, attenuation))
                } else {
                    None
                }
            }
        }
    }
}