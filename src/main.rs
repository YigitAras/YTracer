use std::sync::Arc;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

use kdam::tqdm;
use rand::Rng;
use rayon::prelude::*;

use crate::texture::CheckerTexture;
use crate::{
    bvh::*, camera::*, hittable::*, hittable_list::*, material::*, ray::*, sphere::*, texture::*,
    utils::*, vector3::*,
};
use crate::rect::XYRect;

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod perlin;
mod ray;
mod rect;
mod sphere;
mod texture;
mod utils;
mod vector3;

fn ray_color(r: Ray, background: Vec3, world: &dyn Hittable, depth: u64) -> Vec3 {
    // Depth limit reached don't accumulate any more light
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::MAX) {
        let emitted = hit.mat_ptr.emitted(hit.u, hit.v, hit.p);
        if let Some((scattered, attenuation)) = hit.mat_ptr.scatter(r, &hit) {
            // eprintln!("We do hit something and get some color");
            emitted + ray_color(scattered, background, world, depth - 1) * attenuation
        } else {
            emitted
        }
    } else {
        // If not hit anything
        background
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world: HittableList = Default::default();
    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::from_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let material_ground: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_texture(Arc::clone(&checker)));

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
                        Arc::new(Lambertian::from_color(albedo));
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
    let mat2: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
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

fn checker_world() -> HittableList {
    let mut world: HittableList = Default::default();
    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::from_color(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let mat_checker: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::from_texture(checker));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&mat_checker),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::clone(&mat_checker),
    )));
    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world: HittableList = Default::default();
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
    let mat_perlin: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::from_texture(pertext));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&mat_perlin),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&mat_perlin),
    )));

    world
}

fn earth_scene() -> HittableList {
    let mut world: HittableList = Default::default();
    let image = image::open("static/moon.jpg")
        .expect("image not found")
        .to_rgb8();
    let (width, height) = image.dimensions();
    let data = image.into_raw();
    let texture: Arc<dyn Texture + Sync + Send> =
        Arc::new(ImageTexture::new(data, width as u64, height as u64));
    let mat_world: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::from_texture(texture));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::clone(&mat_world),
    )));

    world
}

fn simple_light() -> HittableList {
    let mut world: HittableList = Default::default();
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
    let mat_perlin: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::from_texture(pertext));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&mat_perlin),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&mat_perlin),
    )));

    let diff_light: Arc<dyn Material + Sync + Send> = Arc::new(
        DiffuseLight::from_color(Vec3::new(4.0,4.0,4.0))
    );

    world.add(Arc::new(XYRect::new(
        3.0, 5.0, 1.0, 3.0, -2.0, diff_light
    )));
    world
}

fn main() {
    // IF DEBUG:
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();

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
    let world: Bvh;

    let lookfrom: Vec3;
    let lookat: Vec3;
    let vfov: f64;
    let mut aperture = 0.0;
    let mut background = Vec3::new(0.0, 0.0, 0.0);

    // Select World to Render
    let scene_id: u8 = 4;
    let mut items: HittableList;
    match scene_id {
        0 => {
            items = random_scene();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        1 => {
            items = checker_world();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        2 => {
            items = two_perlin_spheres();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            items = earth_scene();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            items = simple_light();
            background = Vec3::new(0.0,0.0,0.0);
            lookfrom = Vec3::new(26.0,3.0,6.0);
            lookat = Vec3::new(0.0,2.0,0.0);
            vfov = 20.0;
        }
        _ => panic!["Unimplemented scene code!"],
    }

    let list_len = items.objects.len();
    world = Bvh::new(&mut items, 0, list_len, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    // For the bigger world
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
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
                        col += ray_color(r, background, &world, DEPTH);
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

    println!("\nNow writing the values into the PPM file...\n");
    for r in rows {
        for col in r {
            let ir = col.x as u64;
            let ig = col.y as u64;
            let ib = col.z as u64;
            let tmp_data: String = format!("{ir} {ig} {ib}\n");
            file.write_all(tmp_data.as_bytes())
                .expect("Failed to write line of pixel data...");
        }
    }

    println!("Program ended...\n");
}
