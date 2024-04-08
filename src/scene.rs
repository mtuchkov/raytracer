use surfaces::world::World;
use crate::scene::camera::Camera;

pub(crate) mod builder;
pub mod surfaces;
pub mod material;
pub(crate) mod camera;

pub(crate) struct Scene {
    pub(crate) camera: Camera,
    pub(crate) world: World,
    pub(crate) w: i32,
    pub(crate) h: i32,
}

impl Scene {
    pub(crate) fn camera(&self) -> &Camera {
        &self.camera
    }
    pub(crate) fn world(&self) -> &World {
        &self.world
    }
}
