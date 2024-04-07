use crate::surfaces::hitable::{Hitable, HitRecord};
use crate::surfaces::Surface;
use crate::vec::Ray;

pub(crate) struct World {
    objects: Vec<Surface>,
    size: usize,
}

impl World {
    pub(crate) fn new() -> World {
        World {
            objects: Vec::new(),
            size: 0,
        }
    }

    pub(crate) fn add(&mut self, object: Surface) {
        self.objects.push(object);
        self.size += 1;
    }
}

impl Hitable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;

        let mut closest_so_far = t_max;

        // We cannot use the monadic behavior here as we need to update the closest value.
        for hitable in self.objects.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec = Some(rec);
                },
                None => {},
            }
        }

        temp_rec
    }
}