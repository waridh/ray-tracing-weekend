use crate::{color::Color, hittable, ray::Ray, vec3::Vec3};
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
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let scatter_dir = match hit_rec.normal + Vec3::random_unit_vector() {
            x if x.near_zero() => hit_rec.normal,
            x => x,
        };
        let r_out = Ray::new(scatter_dir, hit_rec.p);
        Some((self.albedo.clone(), r_out))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let reflection_dir =
            r_in.direction.reflect(&hit_rec.normal) + (self.fuzz * Vec3::random_unit_vector());

        // Catching degenerate scatter direction
        if reflection_dir.dot(&(hit_rec.normal)) > 0. {
            let r_out = Ray::new(reflection_dir, hit_rec.p);
            Some((self.albedo.clone(), r_out))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub struct Dielectric {
    refractive_index: f32,
    attenuation: Color,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        let attenuation = Color::new(1., 1., 1.);
        Dielectric {
            refractive_index,
            attenuation,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let refractive_index = if hit_rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let refracted = r_in.direction.refract(&hit_rec.normal, refractive_index);
        Some((self.attenuation.clone(), Ray::new(refracted, hit_rec.p)))
    }
}
