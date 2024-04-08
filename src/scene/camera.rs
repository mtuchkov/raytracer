use crate::math::vec::{Ray, Vec3};

pub(crate) enum Camera {
    StaticCamera {
        origin: Vec3,
        ll_corner: Vec3,
        horizontal: Vec3,
        vertical: Vec3,
    },
    PositionableCamera {
        origin: Vec3,
        ll_corner: Vec3,
        horizontal: Vec3,
        vertical: Vec3,
        u: Vec3,
        v: Vec3,
        lens_radius: f32,
    },
}

pub(crate) trait RaySource {
    fn get_ray(&self, s: f32, t: f32) -> Ray;
}

impl Camera {
    pub(crate) fn positionable(
                    look_from: Vec3,
                    look_at: Vec3,
                    up: Vec3,
                    vfov: f32,
                    aspect: f32,
                    aperture: f32,
                    focus_dist: f32) -> Camera {

        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // LEARN:
        let w = (&look_from - &look_at).unit();
        //             ^
        // If we would not use ampersand here, the compiler would complain:
        //
        // error[E0382]: use of moved value: `look_from`
        // let w = (look_from - &look_at).unit();
        //          ---------------------- `look_from` moved due to usage in operator
        // ll_corner: &look_from - half_width * &u - half_height * &v - &w,
        //            ^^^^^^^^^^ value borrowed here after mov
        //
        // This is a VERY powerful feature of Rust
        // E.g. we can implement algebraic operations in a way that they consume
        // the operands or not and worry less about the ownership issues.
        let u = Vec3::cross(&up, &w).unit();
        let v = Vec3::cross(&w, &u);
        Camera::PositionableCamera {
            ll_corner: &look_from - half_width * focus_dist * &u - half_height * focus_dist * &v - &w,
            horizontal: 2. * half_width * focus_dist * &u,
            vertical: 2. * half_height * focus_dist * &v,
            origin: look_from,
            // LEARN: you don't need to specify the name of the field if it can be inferred
            v,
            u,
            lens_radius: aperture / 2.,
        }
    }

    pub(crate) fn static_camera() -> Camera {
        Camera::StaticCamera {
            ll_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
            origin: Vec3::new(0., 0., 0.),
        }
    }
}

impl RaySource for Camera {


    fn get_ray(&self, s: f32, t: f32) -> Ray {
        match self {
            Camera::StaticCamera { origin, ll_corner, horizontal, vertical } => {
                let ray_origin:Vec3 = origin.clone();
                Ray::from(ray_origin, ll_corner + s * horizontal + t * vertical - origin)
            }
            Camera::PositionableCamera { origin, ll_corner, horizontal, vertical, u, v, lens_radius } => {
                let rand = *lens_radius * Vec3::random_in_unit_disk();
                let offset = u * rand.x() + v * rand.y();
                let direction = ll_corner
                    + s * horizontal
                    + t * vertical
                    - origin
                    - &offset;
                let origin = origin + &offset;

                Ray::from(origin, direction)
            }
        }
    }
}