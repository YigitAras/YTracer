use kdam::tqdm;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vector3;

use crate::{
    bvh::*, camera::*, hittable::*, hittable_list::*, material::*, ray::*, sphere::*, utils::*,
    vector3::*,
};

fn ray_color(r: Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: u64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.00001, f64::MAX) {
        if let Some((scattered, attenuation)) = hit.mat_ptr.scatter(r, &hit) {
            return ray_color(scattered, world, rng, depth - 1) * attenuation;
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn random_vec(l: f64, h: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3 {
        x: rng.gen_range(l..h),
        y: rng.gen_range(l..h),
        z: rng.gen_range(l..h),
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world: HittableList = Default::default();
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material_ground),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - (Vec3::new(4.0, 0.2, 0.0))).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vec(0.0, 1.0) * random_vec(0.0, 1.0);
                    let sphere_mat: Arc<dyn Material + Sync + Send> =
                        Arc::new(Lambertian { albedo });
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::clone(&sphere_mat))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec(0.5, 1.0);
                    let fuzz = rng.gen::<f64>();
                    let sphere_mat: Arc<dyn Material + Sync + Send> =
                        Arc::new(Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::clone(&sphere_mat))));
                } else {
                    // glass
                    let sphere_mat: Arc<dyn Material + Sync + Send> =
                        Arc::new(Dielectric { ir: 1.5 });
                    world.add(Arc::new(Sphere::new(center, 0.2, Arc::clone(&sphere_mat))));
                }
            }
        }
    }
    let mat1: Arc<dyn Material + Sync + Send> = Arc::new(Dielectric { ir: 1.5 });
    let mat2: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let mat3: Arc<dyn Material + Sync + Send> = Arc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat1),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat2),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat3),
    )));
    world
}

fn main() {
    println!("Program started...\n");
    // Set number of threads
    // let mut rng = rand::thread_rng();

    // Image related
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const DEPTH: u64 = 50;
    // World
    let world: HittableList = random_scene();

    /*
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    */

    // Camera
    let aperture = 0.1;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
    let rows: Vec<Vec<Vec3>> = tqdm!((0..IMAGE_HEIGHT).rev(), animation = "fillup")
        .map(|j| {
            (0..IMAGE_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut rng = rand::thread_rng();
                    let mut col = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT as f64;
                        let r = cam.get_ray(u, v);
                        col += ray_color(r, &world, &mut rng, DEPTH);
                    }
                    col /= SAMPLES_PER_PIXEL as f64;
                    col = Vec3::new(f64::sqrt(col.x), f64::sqrt(col.y), f64::sqrt(col.z));
                    col.x = 256.0 * clamp(col.x, 0.0, 0.999);
                    col.y = 256.0 * clamp(col.y, 0.0, 0.999);
                    col.z = 256.0 * clamp(col.z, 0.0, 0.999);
                    col
                })
                .collect()
        })
        .collect();

    println!("Now writing the values into the PPM file...\n");
    for r in rows {
        for col in r {
            let ir = col.x as u64;
            let ig = col.y as u64;
            let ib = col.z as u64;
            let tmp_data: String = format!("{ir} {ig} {ib}\n");
            file.write(tmp_data.as_bytes())
                .expect("Failed to write line of pixel data...");
        }
    }

    println!("Program ended...\n");
}
