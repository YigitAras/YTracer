use kdam::tqdm;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

mod color;
mod ray;
mod vector3;

use crate::color::*;
use crate::ray::*;
use crate::vector3::*;



fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool{
    let oc: Vec3 = r.orig - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}


fn main() {
    println!("Program started...\n");
    // Image related
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGHT: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGHT);

    // File
    std::fs::create_dir_all("./outputs")
        .expect("Problem with creation of the outputs/ folder...\n");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("./outputs/IMAGE.ppm")
        .expect("Unable to open the file IMAGE.ppm");

    // Buffered writer for speed
    // Can keep everything in memory too
    let mut file = BufWriter::new(file);
    let data = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    file.write_all(data.as_bytes())
        .expect("Unable to write the header!");

    // Render
    for j in tqdm!((0..IMAGE_HEIGHT).rev(), animation = "fillup") {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(r);
            write_color(&mut file, pixel_color);
        }
    }
    println!("Program ended...\n");
}
