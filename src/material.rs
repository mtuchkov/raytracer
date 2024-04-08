use crate::ffi::drand32;
use crate::surfaces::hitable::HitRecord;
use crate::vec::{Ray, Vec3};

pub(crate) enum Material {
    // randomly diffuses the light
    Lambertian {
        albedo: Vec3,
    },
    // not transparent material that reflects the light
    Metal {
        albedo: Vec3,
        fuzz: f32,
    },
    // transparent material that refracts and also reflects the light
    Dielectric {
        ref_idx: f32,
        // Since glass absorbs no light the attenuation is always 1.0
        // for dielectric material. Let's make a parameter of the material.
        // We could also make it a constant, but we may want to experiment
        // with transparency.
        attenuation: Vec3,
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
    pub(crate) fn dielectric(ref_idx: f32) -> Material {
        Material::Dielectric {
            ref_idx,
            attenuation: Vec3::new(1.0, 1.0, 1.0)
        }
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

        fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
            v - &(2.0 * Vec3::dot(v, n) * n)
        }

        fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
            let uv = v.unit();
            let dt = Vec3::dot(&uv, n);
            let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
            if discriminant > 0.0 {
                let refracted = ni_over_nt * (&uv - &(n * dt)) - &(n * discriminant.sqrt());
                Some(refracted)
            } else {
                None
            }
        }

        // reflection function for dielectric material
        fn schlick(cosine: f32, ref_idx: f32) -> f32 {
            let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }

        // LEARN:
        // breaking the match into separate functions makes the code more readable.
        // however, here we just wanted to demonstrate the local functions.
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
            Material::Dielectric {ref_idx, attenuation} => {

                let outward_normal: Vec3;
                let reflected = reflect(&r_in.direction(), &rec.normal);
                let ni_over_nt: f32;
                let cosine: f32;

                if Vec3::dot(r_in.direction(), &rec.normal) > 0.0 {
                    ni_over_nt = *ref_idx;
                    cosine = *ref_idx * Vec3::dot(r_in.direction(), &rec.normal) / r_in.direction().length();
                    outward_normal = -&rec.normal;
                } else {
                    ni_over_nt = 1.0 / *ref_idx;
                    cosine = -Vec3::dot(r_in.direction(), &rec.normal) / r_in.direction().length();
                    outward_normal = rec.normal;
                }

                // LEARN:
                // You may notice the control flow is different from the C++ code in the book.
                // One of the reasons is that the compiler forces to structure the code in a way
                // that the ownership of the variables is clear and the destructing or consuming
                // operations move toward to the tail of the scope of the variables.

                match refract(&r_in.direction(), &outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        // some rays are reflected and some are refracted
                        // depends on the angle of view
                        if drand32() >= schlick(cosine, *ref_idx) {
                            Some((Ray::from(rec.p, refracted), attenuation))
                        } else {
                            Some((Ray::from(rec.p, reflected), attenuation))
                        }
                    },
                    None => Some((Ray::from(rec.p, reflected), attenuation)),
                }
            }
        }
    }
}