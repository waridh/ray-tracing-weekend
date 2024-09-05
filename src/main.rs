use vec3::Vec3;

use crate::{
    hittable::{Hittable, HittableList},
    material::Material,
};
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn make_basic_world() -> HittableList {
    let mut world = HittableList::default();

    // Materials

    let material_ground = material::Lambertian::new(color::Color::new(0.8, 0.8, 0.));
    let material_center = material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5));
    let material_glass = material::Dielectric::new(1.5);
    let air_pocket = material::Dielectric::new(1. / 1.5);
    let material_right = material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 0.5);
    let metal_shiny = material::Metal::new(color::Color::new(1., 1., 1.), 0.8);

    let sphere_1 = sphere::Sphere::new(Vec3(0., 0., -1.), 0.5, material_center);
    let sphere_2 = sphere::Sphere::new(Vec3::new(1.0, -1000.5, -1.), 1000., material_ground);
    let sphere_3 = sphere::Sphere::new(Vec3::new(-0.9, -0.25, -1.), 0.25, material_glass);
    let sphere_3_inner = sphere::Sphere::new(Vec3::new(-0.9, -0.25, -1.), 0.125, air_pocket);
    let sphere_4 = sphere::Sphere::new(Vec3::new(1., 0., -1.2), 0.5, material_right);
    let sphere_5 = sphere::Sphere::new(Vec3::new(0., 0.75, -1.), 0.25, metal_shiny);

    world.push(sphere_1);
    world.push(sphere_3);
    world.push(sphere_3_inner);
    world.push(sphere_4);
    world.push(sphere_2);
    world.push(sphere_5);
    world
}

fn main() {
    let aspect_ratio: f32 = 5. / 4.;
    let image_width = 600usize;
    let samples_per_pixel = 100;
    let reflection_depth = 50;

    let mut camera = camera::Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        reflection_depth,
        30.,
        Vec3(-2., 2., 1.),
        Vec3(0., 0., -1.),
        Vec3(0., 1., 0.),
    );

    // World
    let world = make_basic_world();

    camera.render(&world);
}
