use crate::{color, hittable::Hittable, ray, vec3};
use indicatif::ProgressIterator;
use std::{f32::INFINITY, ops::Range, rc::Rc};

const ZERO_TO_INFINITY: Range<f32> = 0f32..INFINITY;
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: usize,
    image_height: usize,
    pixel00: vec3::Vec3,
    pixel_delta_u: vec3::Vec3,
    pixel_delta_v: vec3::Vec3,
    center: Rc<vec3::Vec3>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: usize, focal_length: f32) -> Self {
        let image_height = match ((image_width as f32) / aspect_ratio) as usize {
            x if x < 1 => 1,
            x => x,
        };

        let viewport_height = 2f32;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = Rc::from(vec3::Point3::from((0., 0., 0.)));

        // Viewport vectors
        let viewport_u = vec3::Vec3(viewport_width, 0., 0.);
        let viewport_v = vec3::Vec3(0., -viewport_height, 0.);

        // Calculating pixel delta
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Getting the location of the top left corner of the viewport
        let viewport_upper_left: vec3::Vec3 = center.as_ref()
            - vec3::Vec3(0., 0., focal_length)
            - (viewport_u / 2.)
            - (viewport_v / 2.);
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            center,
        }
    }
    /// Reders the scene with side-effects going straight to stdout
    /// A buffer writer would improve the performance of this function
    pub fn render(&self, world: &impl Hittable) {
        // render

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00 + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
                let raydir = pixel_center - self.center.as_ref();
                let r = ray::Ray::new(raydir, &self.center);
                let pixel = Camera::ray_color(r, world);

                println!("{}", pixel);
            }
        }
    }

    fn ray_color(r: ray::Ray, world: &impl Hittable) -> color::Color {
        match world.hit(&r, &ZERO_TO_INFINITY) {
            Some(t) => {
                let m = 0.5 * (vec3::Vec3(1., 1., 1.) + t.normal.unit_vector());
                color::Color::from(m)
            }
            None => {
                let unit_dir = r.direction.unit_vector();
                let a = 0.5 * (unit_dir.1 + 1.);
                color::Color::from_args(1., 1., 1.) * (1. - a)
                    + a * color::Color::from_args(0.8, 0.5, 1.0)
            }
        }
    }
}
