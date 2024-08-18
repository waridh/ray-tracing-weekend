use crate::{
    color::Color,
    hittable::Hittable,
    ray::{self},
    vec3::{self},
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{self, Rng};
use std::f32::INFINITY;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub reflection_depth: usize,
    image_height: usize,
    pixel00: vec3::Vec3,
    pixel_delta_u: vec3::Vec3,
    pixel_delta_v: vec3::Vec3,
    center: vec3::Vec3,
    pixel_sample_scale: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: usize,
        focal_length: f32,
        samples_per_pixel: usize,
        reflection_depth: usize,
    ) -> Self {
        let image_height = match ((image_width as f32) / aspect_ratio) as usize {
            x if x < 1 => 1,
            x => x,
        };

        let viewport_height = 2f32;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = vec3::Point3::from((0., 0., 0.));

        // Viewport vectors
        let viewport_u = vec3::Vec3(viewport_width, 0., 0.);
        let viewport_v = vec3::Vec3(0., -viewport_height, 0.);

        // Calculating pixel delta
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Getting the location of the top left corner of the viewport
        let viewport_upper_left: vec3::Vec3 =
            center - vec3::Vec3(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_sample_scale = 1. / (samples_per_pixel as f32);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            center,
            pixel_sample_scale,
            reflection_depth,
        }
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

    fn get_ray(&mut self, i: usize, j: usize) -> ray::Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel00
            + (self.pixel_delta_u * ((i as f32) + offset[0]))
            + (self.pixel_delta_v * ((j as f32) + offset[1]));
        let raydir = pixel_center - self.center;
        ray::Ray::new(raydir, self.center)
    }

    fn sample_square(&mut self) -> vec3::Vec3 {
        let mut rng = rand::thread_rng();
        vec3::Vec3(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.)
    }

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
                let unit_dir = r.direction.unit_vector();
                let a = 0.5 * (unit_dir.1 + 1.);
                Color::new(1., 1., 1.) * (1. - a) + a * Color::new(0.8, 0.5, 1.0)
            }
        }
    }
}
