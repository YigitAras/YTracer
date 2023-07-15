use kdam::tqdm;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod vector3;

use crate::color::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::vector3::*;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = r.orig - center;
    let a = r.dir.lenght_squared();
    let half_b = oc.dot(r.dir);
    let c = oc.lenght_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn update_record(rec: &mut HitRecord) {
    rec.p = Vec3::new(10.0, 10.0, 10.0);
    rec.normal = Vec3::new(10.0, 10.0, 10.0);
    rec.t = -10.0;
    rec.front_face = false;
}

fn main() {
    println!("Program started...\n");

    println!("Running test on mutable reference...");
    let mut record = HitRecord::new(
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        true,
    );
    println!("Before function: ");
    println!("{:?}", record);
    println!("After function: ");
    update_record(&mut record);
    println!("{:?}", record);

    // Image related
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1080;
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
