use indicatif::ProgressIterator;

pub mod vec3;

fn main() {
    let image_width = 256usize;
    let image_height = 256usize;

    // render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
