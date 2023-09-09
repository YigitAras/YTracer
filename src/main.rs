use crate::{bvh::*, camera::*, hittable_list::*, scene::*, vector3::*};

// General Todo's to implement
// TODO: (1) A class for meshes with one material
// TODO: (2) Implement transforms as matrix objects
// TODO: (3) Analyse and find hot points, make them faster
// TODO: (4) Implement SIMD and Vectorized speed-ups
// TODO: (5) Better API for object, material and texture creation. Too verbose atm...
// TODO: (6) Different Tree implementations

mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod instance;
mod material;
mod mesh;
mod perlin;
mod ray;
mod rect;
mod scene;
mod sphere;
mod texture;
mod triangle;
mod utils;
mod vector3;

fn main() {
    // IF DEBUG:
    rayon::ThreadPoolBuilder::new()
      .num_threads(1)
        .build_global()
       .unwrap();

    println!("Program started...\n");

    // Image related
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u64 = 800;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const DEPTH: u64 = 50;
    // World
    let world;

    let lookfrom: Vec3;
    let lookat: Vec3;
    let vfov: f64;
    let mut _aperture = 0.0;
    let background;

    // Select World to Render
    let scene_id: u8 = 7;
    let mut items: HittableList;
    match scene_id {
        0 => {
            items = Scene::random_scene();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            _aperture = 0.1;
        }
        1 => {
            items = Scene::checker_world();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        2 => {
            items = Scene::two_perlin_spheres();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            items = Scene::earth_scene();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            items = Scene::simple_light();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            items = Scene::cornell_box();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        6 => {
            items = Scene::cornell_with_gas();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            items = Scene::cornell_with_mesh(false);
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => panic!["Unimplemented scene code!"],
    }

    let list_len = items.objects.len();
    world = Bvh::new(&mut items, 0, list_len, 0.0, 0.0);
    // Initialize the camera
    let camera = Camera::init(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        DEPTH,
        vfov,
        lookfrom,
        lookat,
    );

    camera.render(&world, background);
}
