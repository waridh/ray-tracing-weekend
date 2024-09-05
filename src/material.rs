use rand::Rng;

use crate::{color::Color, hittable, ray::Ray, vec3::Vec3};
pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)>;
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

pub struct Dielectric {
    refractive_index: f32,
    attenuation: Color,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        // Currently fixed attenuation of 1
        let attenuation = Color::new(1., 1., 1.);
        Dielectric {
            refractive_index,
            attenuation,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &hittable::HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();
        let unit_r_in_dir = r_in.direction.unit_vector();
        let refractive_index = if hit_rec.front_face {
            // Air into the material
            1.0 / self.refractive_index
        } else {
            // Material into the air
            self.refractive_index
        };

        let cos_theta = ((-unit_r_in_dir).dot(&hit_rec.normal)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refractive_index * sin_theta > 1.;

        let scatter = if cannot_refract
            || reflectance(cos_theta, self.refractive_index) > rng.gen_range(-1f32..1f32)
        {
            unit_r_in_dir.reflect(&hit_rec.normal)
        } else {
            unit_r_in_dir.refract(&hit_rec.normal, refractive_index)
        };

        Some((self.attenuation.clone(), Ray::new(scatter, hit_rec.p)))
    }
}
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r02 = r0 * r0;
    r02 + (1. - r02) * ((1. - cosine).powi(5))
}
