struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut hit_record) -> bool;
}