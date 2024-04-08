use crate::camera::Camera;
use crate::color::Color;
use crate::ffi::drand32;
use crate::material::Material;
use crate::scene::Scene;
use crate::surfaces::Surface;
use crate::surfaces::world::World;
use crate::vec::Vec3;

pub(crate) trait SceneBuilder {
    fn build(&self) -> Scene;
}

pub(crate) enum BuiltIn {
    Default,
    Random,
}

impl SceneBuilder for BuiltIn {
    fn build(&self) -> Scene {
        match self {
            BuiltIn::Default => self.buidl_default_scene(),
            BuiltIn::Random => self.build_random_scene()
        }
    }
}

impl BuiltIn {

    pub(crate) fn default() -> BuiltIn {
        BuiltIn::Default
    }
    pub(crate) fn random() -> BuiltIn {
        BuiltIn::Random
    }

    fn buidl_default_scene(&self) -> Scene {
        Scene {
            camera: self.create_camera(200, 100),
            world: self.create_default_world(),
            w: 200,
            h: 100,
        }
    }

    fn build_random_scene(&self) -> Scene {
        Scene {
            camera: self.create_camera(200, 100),
            world: self.create_random_world(),
            w: 1024,
            h: 512,
        }
    }

    fn create_camera(&self, w: i32, h: i32) -> Camera {
        // LEARN:
        // float declaration can omit the trailing zeros, e.g. 0.0 -> 0.
        // Rust does not allow the implicit cast, so this is not a problem in the code review.
        let look_from = Vec3::new(2., 2., 0.);
        let look_at = Vec3::new(0., 0., -1.);
        let dist_to_focus = (&look_from - &look_at).length();
        let up = Vec3::new(0., 1., 0.);
        let fov = 20.;
        let aspect = w as f32 / h as f32;
        let aperture= 0.1;

        Camera::new(
            look_from,
            look_at,
            up,
            fov,
            aspect,
            aperture,
            dist_to_focus)
    }

    fn create_default_world(&self) -> World {

        let mut world = World::new();

        world.add(
            Surface::sphere(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Material::lambertian(Vec3::rgb(0.1, 0.2, 0.5))));
        world.add(
            Surface::sphere(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Material::lambertian(Vec3::rgb(0.8, 0.8, 0.0))));
        world.add(
            Surface::sphere(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Material::metal(Vec3::rgb(0.8, 0.6, 0.2), 0.2)));
        world.add(
            Surface::sphere(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Material::dielectric(1.5)));
        // trick with negative radius does not affect the geometry and makes the sphere hollow
        world.add(
            Surface::sphere(
                Vec3::new(-1.0, 0.0, -1.0),
                -0.45,
                Material::dielectric(1.5)));
        world
    }

    fn create_random_world(&self) -> World {

        let mut world = World::new();

        world.add(
            Surface::sphere(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                Material::lambertian(Vec3::rgb(0.5, 0.5, 0.5))));

        for a in -11..11 {
            for b in -11..11 {
                let material = (drand32() * 100.) as i32;
                assert!(material >= 0 && material < 100, "Material index out of range");
                let center = Vec3::new(a as f32 + 0.9 * drand32(), 0.2, b as f32 + 0.9 * drand32());
                if (&center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                    match material {
                        // 80% is diffuse
                        0..=79 => {
                            let albedo = Vec3::rgb(drand32() * drand32(),
                                                   drand32() * drand32(),
                                                   drand32() * drand32());
                            world.add(Surface::sphere(center, 0.2, Material::lambertian(albedo)));
                        }
                        // 15% is metal
                        80..=94 => {
                            let albedo = Vec3::rgb(
                                0.5 * (1. + drand32() * drand32()),
                                0.5 * (1. + drand32() * drand32()),
                                0.5 * (1. + drand32() * drand32()), );
                            world.add(
                                Surface::sphere(
                                    center,
                                    0.2,
                                    Material::metal(albedo, 0.5 * drand32())));
                        }
                        // 5% is glass
                        _ => {
                            world.add(
                                Surface::sphere(
                                    center,
                                    0.2,
                                    Material::dielectric(1.5)));
                        }
                    }
                }
            }
        }

        world.add(
            Surface::sphere(
                Vec3::new(0.0, 1.0, 0.0),
                1.0,
                Material::dielectric(1.5)));
        world.add(
            Surface::sphere(
                Vec3::new(-4.0, 1.0, 0.0),
                1.0,
                Material::lambertian(Vec3::rgb(0.4, 0.2, 0.1))));
        world.add(
            Surface::sphere(
                Vec3::new(4.0, 1.0, 0.0),
                1.0,
                Material::metal(Vec3::rgb(0.7, 0.6, 0.5), 0.0)));

        world
    }
}