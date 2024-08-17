use crate::{color, hittable::Hittable, ray, vec3};
use indicatif::ProgressIterator;
use std::{f32::INFINITY, ops::Range, rc::Rc};

const ZERO_TO_INFINITY: Range<f32> = 0f32..INFINITY;
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: usize,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1f32,
            image_width: 100,
        }
    }
}

impl Camera {
    /// Reders the scene with side-effects going straight to stdout
    /// A buffer writer would improve the performance of this function
    pub fn render(&self, world: &impl Hittable) {
        // Unclear currently if this could be an if let.
        let image_height_: usize = ((self.image_width as f32) / self.aspect_ratio) as usize;
        let image_height: usize = if image_height_ < 1 { 1 } else { image_height_ };
        let focal_length = 1.0;
        let viewport_height = 2f32;
        let viewport_width = viewport_height * (self.image_width as f32 / image_height as f32);
        let camera_center = Rc::from(vec3::Point3::from((0., 0., 0.)));

        // Viewport vectors
        let viewport_u = vec3::Vec3(viewport_width, 0., 0.);
        let viewport_v = vec3::Vec3(0., -viewport_height, 0.);

        // Calculating pixel delta
        let pixel_delta_u = viewport_u / self.image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Getting the location of the top left corner of the viewport
        let viewport_upper_left: vec3::Vec3 = camera_center.as_ref()
            - vec3::Vec3(0., 0., focal_length)
            - (viewport_u / 2.)
            - (viewport_v / 2.);
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // render

        println!("P3\n{} {}\n255", self.image_width, image_height);

        for j in (0..image_height).progress() {
            for i in 0..self.image_width {
                let pixel_center = pixel00 + (pixel_delta_u * i) + (pixel_delta_v * j);
                let raydir = pixel_center - camera_center.as_ref();
                let r = ray::Ray::new(raydir, &camera_center);
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
