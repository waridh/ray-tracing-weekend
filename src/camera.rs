use crate::{
    color::Color,
    hittable::Hittable,
    ray::{self},
    vec3::{self, Point3, Vec3},
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{self, Rng};
use rayon::prelude::*;
use std::f32::INFINITY;

pub struct CameraBuilder {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub reflection_depth: usize,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_to: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}
impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 16. / 9.,
            image_width: 1200,
            samples_per_pixel: 100,
            reflection_depth: 50,
            vfov: 90.,
            look_from: Vec3::new(0., 0., 0.),
            look_to: Vec3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_distance: 10.,
        }
    }
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let image_height = match ((self.image_width as f32) / self.aspect_ratio) as usize {
            x if x < 1 => 1,
            x => x,
        };
        let pixel_sample_scale = 1. / (self.samples_per_pixel as f32);
        let center = self.look_from;

        let fov_theta = self.vfov.to_radians();
        let h = (fov_theta / 2.).tan();

        let viewport_height = 2. * h * self.focus_distance;
        let viewport_width = viewport_height * ((self.image_width as f32) / (image_height as f32));

        let w = (center - self.look_to).normalize();
        let u = self.vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Viewport vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculating pixel delta
        let pixel_delta_u = viewport_u / self.image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Getting the location of the top left corner of the viewport
        let viewport_upper_left =
            center - (self.focus_distance * w) - (viewport_u / 2.) - (viewport_v / 2.);
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let lens_dimensions = if self.defocus_angle > 0. {
            // Camera defocus disk vector
            let defocus_radius = self.focus_distance * (self.defocus_angle / 2.).to_radians().tan();
            let lens_u = defocus_radius * u;
            let lens_v = defocus_radius * v;
            Some((lens_u, lens_v))
        } else {
            None
        };

        Camera {
            image_width: self.image_width,
            samples_per_pixel: self.samples_per_pixel,
            image_height,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            center,
            pixel_sample_scale,
            reflection_depth: self.reflection_depth,
            lens_dimensions,
        }
    }
}

pub struct Camera {
    image_width: usize,
    samples_per_pixel: usize,
    reflection_depth: usize,
    image_height: usize,
    pixel00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Vec3,
    pixel_sample_scale: f32,
    lens_dimensions: Option<(Vec3, Vec3)>,
}

impl Camera {
    /// Generates the builder object
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }
    /// Renders the scene with side-effects going straight to stdout
    /// A buffer writer would improve the performance of this function
    pub fn render(&mut self, world: &impl Hittable) {
        // render
        let bar = ProgressBar::new(self.image_height as u64);
        let prog_style = ProgressStyle::with_template(
            "[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg} [eta {eta_precise}]",
        )
        .unwrap()
        .tick_chars("#+-");
        bar.set_style(prog_style);

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel += Camera::ray_color(r, self.reflection_depth, world);
                }

                println!("{}", pixel * self.pixel_sample_scale);
            }
            bar.inc(1);
        }
        bar.finish();
    }

    /// Create a ray from the defocus lens in the camera center, and direct
    /// it at the pixel square
    fn get_ray(&mut self, i: usize, j: usize) -> ray::Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel00
            + (self.pixel_delta_u * ((i as f32) + offset[0]))
            + (self.pixel_delta_v * ((j as f32) + offset[1]));
        let ray_orig = match self.lens_dimensions {
            None => self.center,
            Some((lens_u, lens_v)) => {
                let p = Vec3::random_in_unit_disk();
                self.center + (p[0] * lens_u) + (p[1] * lens_v)
            }
        };
        let raydir = pixel_center - ray_orig;

        ray::Ray::new(raydir, ray_orig)
    }

    fn sample_square(&mut self) -> vec3::Vec3 {
        let mut rng = rand::thread_rng();
        vec3::Vec3(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.)
    }

    /// Retrieve the final color of a ray traversing through the world
    fn ray_color(r: ray::Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::black();
        }
        match world.hit(&r, &(0.001..INFINITY)) {
            Some(t) => match t.material.scatter(&r, &t) {
                Some((attenuation, scattered)) => {
                    attenuation * Camera::ray_color(scattered, depth - 1, world)
                }
                None => Color::black(),
            },
            None => {
                // This is the background branch
                let unit_dir = r.direction.normalize();
                let a = 0.5 * (unit_dir.1 + 1.);
                Color::new(1., 1., 1.) * (1. - a) + a * Color::new(0.6, 0.5, 1.0)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
