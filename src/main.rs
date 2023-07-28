use kdam::tqdm;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vector3;

use crate::camera::*;
use crate::color::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utils::*;
use crate::vector3::*;

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

fn main() {
    println!("Program started...\n");
    // Set number of threads
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();
    let mut rng = rand::thread_rng();

    // Image related
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1080;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 200;
    const DEPTH: u64 = 50;
    // World
    let mut world: HittableList = Default::default();
    let R = f64::cos(std::f64::consts::PI/4.0);
    
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left: Arc<dyn Material + Sync + Send> = Arc::new(Dielectric { ir: 1.5 });
    let material_right: Arc<dyn Material + Sync + Send> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        // gotta clone the rc here if you want to reuse it later another time
        Arc::clone(&material_left),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Arc::clone(&material_left),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_right),
    )));
   

    // Camera
    
    let cam = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0, 
        ASPECT_RATIO);

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

    let img: [[Vec3; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];

    file.write_all(data.as_bytes())
        .expect("Unable to write the header!");

    // Render
    for j in tqdm!((0..IMAGE_HEIGHT).rev(), animation = "fillup") {
        for i in 0..IMAGE_WIDTH {
            // let mut sum = Vec3::new(0.0,0.0,0.0);

            /* 
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / ((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                sum += ray_color(r, &world, &mut rng, DEPTH - 1);
            }*/
            
            let sum: Vec3 = 
                (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_|
                    {
                        let mut rng = rand::thread_rng();
                        let u = (i as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                        let v = (j as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                        let ray = cam.get_ray(u, v);
                        ray_color(ray, &world, &mut rng, DEPTH)
                    }
                )
                .reduce(|| Vec3::new(0.0,0.0,0.0), |sum, i| sum + i);
                
        
            
            

            write_color(&mut file, sum, SAMPLES_PER_PIXEL);
        }
    }
    println!("Program ended...\n");
}
