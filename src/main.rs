use std::rc::Rc;

mod camera;
mod color;
mod hittable;
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
