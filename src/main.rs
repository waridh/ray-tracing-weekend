use vec3::Vec3;

use crate::{hittable::Hittable, material::Material};
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio: f32 = 5. / 4.;
    let image_width = 1200usize;
    let samples_per_pixel = 100;
    let reflection_depth = 50;

    let mut camera = camera::Camera::new(
        aspect_ratio,
        image_width,
        1.0,
        samples_per_pixel,
        reflection_depth,
    );

    // Materials

    let material_ground: Rc<dyn Material> =
        Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.)));
    let material_center: Rc<dyn Material> =
        Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> =
        Rc::new(material::Metal::new(color::Color::new(0.8, 0.8, 0.8), 0.2));
    let material_right: Rc<dyn Material> =
        Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.0));

    // World

    let sphere_1: Rc<dyn Hittable> = Rc::new(sphere::Sphere::new(
        Vec3(0., 0., -1.),
        0.5,
        &material_center,
    ));
    let sphere_2: Rc<dyn Hittable> = Rc::new(sphere::Sphere::new(
        Vec3::new(1.0, -100.5, -1.),
        100.,
        &material_ground,
    ));
    let sphere_3: Rc<dyn Hittable> = Rc::new(sphere::Sphere::new(
        Vec3::new(-1., -0.25, -1.),
        0.25,
        &material_left,
    ));
    let sphere_4: Rc<dyn Hittable> = Rc::new(sphere::Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        &material_right,
    ));
    let mut world = hittable::HittableList::new();

    world.push(&sphere_1);
    world.push(&sphere_3);
    world.push(&sphere_4);
    world.push(&sphere_2);

    camera.render(&world);
}
