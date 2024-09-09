use anyhow::{anyhow, Result};
use material::Lambertian;
use vec3::Vec3;

use crate::{color::Color, hittable::HittableList, sphere::Sphere};
use rand::{self, Rng};

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "raytracerust")]
#[command(version = "0.1.0")]
#[command(about="CLI program that generates a ray tracing image", long_about=None)]
pub struct Args {
    /// Output destination path. If not provided, the program will send the
    /// output into stdout
    #[arg(short = 'o', long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

/// Basic world configuration used in the ray tracing in a weekend book
#[allow(dead_code)]
fn make_basic_world() -> HittableList {
    let mut world = HittableList::default();

    // Materials
    let material_ground = material::Lambertian::new(color::Color::new(0.8, 0.8, 0.));
    let material_center = material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5));
    let material_glass = material::Dielectric::new(1.5);
    let air_pocket = material::Dielectric::new(1. / 1.5);
    let material_right = material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.);

    // Objects
    let sphere_1 = sphere::Sphere::new(Vec3(0., 0., -1.2), 0.5, material_center);
    let sphere_3 = sphere::Sphere::new(Vec3::new(-0.9, -0.25, -1.), 0.5, material_glass);
    let sphere_3_inner = sphere::Sphere::new(Vec3::new(-0.9, -0.25, -1.), 0.4, air_pocket);
    let sphere_4 = sphere::Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right);

    // Inserts into the world
    world.push(sphere_1);
    world.push(sphere_3);
    world.push(sphere_3_inner);
    world.push(sphere_4);
    world.push(sphere::Sphere::new(
        Vec3::new(1.0, -100.5, -1.),
        100.,
        material_ground,
    ));

    world
}

#[allow(dead_code)]
fn make_wide_angle_world() -> HittableList {
    let mut world = HittableList::default();

    let r = (std::f32::consts::FRAC_PI_4).cos();

    let material_left = Lambertian::new(Color::new(0., 0., 1.));
    let material_right = Lambertian::new(Color::new(1., 0., 0.));

    world.push(Sphere::new(Vec3::new(-r, 0., -1.), r, material_left));
    world.push(Sphere::new(Vec3::new(r, 0., -1.), r, material_right));

    world
}

fn make_random_world() -> hittable::HittableList {
    let mut rng = rand::thread_rng();
    let mut world = hittable::HittableList::default();

    // Floor
    let material_ground = material::Lambertian::new(Color::new(0.8, 0.8, 0.));
    let floor = sphere::Sphere::new(Vec3::new(0., -1000.5, 0.), 1000., material_ground);
    world.push(floor);

    let ball_rad = 0.2;
    for a in -11..11 {
        for b in -11..11 {
            let rand_mat = rng.gen_range(0f32..1.);
            let center = Vec3::new(
                (a as f32) + 0.9 * rng.gen_range(0f32..1.),
                ball_rad,
                (b as f32) + 0.9 * rng.gen_range(0f32..1.),
            );

            if (center - Vec3::new(4., ball_rad, 0.)).magnitude() > 0.9 {
                match rand_mat {
                    x if x < 0.8 => {
                        let albedo = Color::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        );
                        world.push(Sphere::new(
                            center,
                            ball_rad,
                            material::Lambertian::new(albedo),
                        ))
                    }
                    x if x < 0.95 => {
                        let albedo = color::Color::new(
                            rng.gen_range(0.5..1.),
                            rng.gen_range(0.5..1.),
                            rng.gen_range(0.5..1.),
                        );
                        let fuzz = rng.gen_range(0f32..0.5);
                        world.push(Sphere::new(
                            center,
                            ball_rad,
                            material::Metal::new(albedo, fuzz),
                        ))
                    }
                    _ => world.push(Sphere::new(
                        center,
                        ball_rad,
                        material::Dielectric::new(1.5),
                    )),
                }
            }
        }
    }
    let material_lambertian = material::Lambertian::new(color::Color::new(0.4, 0.2, 0.1));
    let material_glass = material::Dielectric::new(1.5);
    let material_metal = material::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0);

    world.push(Sphere::new(Vec3::new(-4., 1., 0.), 1., material_lambertian));
    world.push(Sphere::new(Vec3::new(4., 1., 0.), 1., material_metal));
    world.push(Sphere::new(Vec3::new(0., 1., 0.), 1., material_glass));

    world
}

fn run(args: &Args) -> Result<()> {
    // Configure camera
    // TODO: Move this logic out to its own function
    let mut camera_builder = camera::Camera::builder();
    camera_builder.aspect_ratio = 5. / 4.;
    camera_builder.image_width = 1200;
    camera_builder.samples_per_pixel = 500;
    camera_builder.reflection_depth = 50;
    camera_builder.vfov = 20.;
    camera_builder.look_from = Vec3(13., 2., 3.);
    camera_builder.look_to = Vec3(0., 0., 0.);
    camera_builder.vup = Vec3(0., 1., 0.);
    camera_builder.defocus_angle = 0.6;
    camera_builder.focus_distance = 10.;

    // World
    let world = make_random_world();
    let mut camera = camera_builder.build();

    match &args.output {
        None => {
            camera.render(&world);
            Ok(())
        }
        Some(x) => Err(anyhow!("Not implemented yet")),
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
