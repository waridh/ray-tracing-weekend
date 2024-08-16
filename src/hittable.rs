use crate::{ray, vec3};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, normal: vec3::Vec3, t: f32, r: &ray::Ray) -> Self {
        let front_face = r.direction.dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}
