use hittable::Hittable;
use indicatif::ProgressIterator;
use std::{f32::INFINITY, ops::Range, rc::Rc};

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

// TODO Remove this const after camera is finished
const ZERO_TO_INFINITY: Range<f32> = 0f32..INFINITY;

/// Calculates the color of the ray.
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

fn main() {
    let aspect_ratio: f32 = 5. / 4.;

    let image_width = 1200usize;

    let camera = camera::Camera {
        aspect_ratio,
        image_width,
    };

    // World
    let sphere_1 = Rc::new(sphere::Sphere::from((0., 0., -1., 0.5)));
    let sphere_2 = Rc::new(sphere::Sphere::from((1.0, -100.5, -1., 100.)));
    let sphere_3 = Rc::new(sphere::Sphere::from((-1., 1., -1., 0.5)));
    let sphere_4 = Rc::new(sphere::Sphere::from((1., 1., -1., 0.5)));
    let mut world = hittable::HittableList::new();

    world.push(&sphere_1);
    world.push(&sphere_3);
    world.push(&sphere_4);
    world.push(&sphere_2);

    camera.render(&world);
}
