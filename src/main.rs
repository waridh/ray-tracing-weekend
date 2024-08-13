use indicatif::ProgressIterator;
use std::rc::Rc;

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: &vec3::Point3, radius: f32, r: &ray::Ray) -> bool {
    let oc = center - r.origin.as_ref();
    let a = r.direction.dot(&r.direction);
    let b = -2.0 * r.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4. * a * c;

    discriminant >= 0.
}

fn ray_color(r: ray::Ray) -> color::Color {
    if hit_sphere(&vec3::Vec3(0., 0., -1.), 0.5, &r) {
        color::Color::from_args(1., 0., 0.)
    } else {
        let unit_dir = r.direction.unit_vector();
        let a = 0.5 * (unit_dir.1 + 1.);
        color::Color::from_args(1., 1., 1.) * (1. - a) + a * color::Color::from_args(0.5, 0.7, 1.0)
    }
}

fn main() {
    let aspect_ratio: f32 = 5. / 4.;

    let image_width = 1200usize;

    // Unclear currently if this could be an if let.
    let image_height_: usize = ((image_width as f32) / aspect_ratio) as usize;
    let image_height = if image_height_ < 1 { 1 } else { image_height_ };

    // Camera
    // TODO: Make the camera a struct
    let focal_length = 1.0;
    let viewport_height = 2f32;
    let viewport_width = viewport_height * ((image_width / image_height) as f32);
    let camera_center = Rc::new(vec3::Point3::from((0., 0., 0.)));

    // Viewport vectors
    let viewport_u = vec3::Vec3(viewport_width, 0., 0.);
    let viewport_v = vec3::Vec3(0., -viewport_height, 0.);

    // Calculating pixel delta
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Getting the location of the top left corner of the viewport
    let viewport_upper_left: vec3::Vec3 = camera_center.as_ref()
        - vec3::Vec3(0., 0., focal_length)
        - (viewport_u / 2.)
        - (viewport_v / 2.);
    let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_center = pixel00 + (pixel_delta_u * i) + (pixel_delta_v * j);
            let raydir = pixel_center - camera_center.as_ref();
            let r = ray::Ray::new(raydir, Rc::clone(&camera_center));
            let pixel = ray_color(r);

            println!("{}", pixel);
        }
    }
}
