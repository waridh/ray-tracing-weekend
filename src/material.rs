use crate::{
    color::{self, Color},
    hittable,
    ray::Ray,
    vec3::Vec3,
};
use std::rc::Rc;
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)>;
}

impl Material for Rc<dyn Material> {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        self.as_ref().scatter(r_in, hit_rec)
    }
}

pub struct Lambertian {
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let scatter_dir = match hit_rec.normal + Vec3::random_unit_vector() {
            x if x.near_zero() => hit_rec.normal,
            x => x,
        };
        let r_out = Ray::new(scatter_dir, hit_rec.p.clone());
        Some((self.albedo.clone(), r_out))
    }
}

pub struct Metal {
    albedo: color::Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let reflection_dir = r_in.direction.reflect(&hit_rec.normal);

        // Catching degenerate scatter direction
        let r_out = Ray::new(reflection_dir, hit_rec.p.clone());
        Some((self.albedo.clone(), r_out))
    }
}
