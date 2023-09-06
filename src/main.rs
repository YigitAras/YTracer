use std::sync::Arc;
use rand::Rng;


use crate::{
    bvh::*, camera::*, hittable::*, hittable_list::*, material::*, rect::*, sphere::*,
    texture::*, utils::*, vector3::*, constant_medium::*, instance::*
};

mod aabb;
mod bvh;
mod constant_medium;
mod hittable;
mod hittable_list;
mod instance;
mod material;
mod perlin;
mod ray;
mod rect;
mod sphere;
mod texture;
mod utils;
mod vector3;
mod camera;
mod triangle;


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

    let diff_light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::from_color(Vec3::new(4.0, 4.0, 4.0)));

    world.add(Arc::new(AARect::new(
        Plane::XY,
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::clone(&diff_light),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 5.0),
        2.0,
        Arc::clone(&diff_light),
    )));

    world
}

fn cornell_box() -> HittableList {
    let mut world: HittableList = Default::default();
    let red: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::from_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    )));
    // nope
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    // Add boxes to the cornell box
    let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    box1 = Arc::new(YRotate::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));
    box2 = Arc::new(YRotate::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    world
}

fn cornell_with_gas() -> HittableList {
    let mut world: HittableList = Default::default();
    let red: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::from_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    )));
    // nope
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    // Add boxes to the cornell box
    let mut box1: Arc<dyn Hittable + Sync + Send> = Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    box1 = Arc::new(YRotate::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable + Sync + Send> = Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));
    box2 = Arc::new(YRotate::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
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
    let scene_id: u8 = 5;
    let mut items: HittableList;
    match scene_id {
        0 => {
            items = random_scene();
            background = Vec3::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            _aperture = 0.1;
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
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            items = cornell_box();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        6 => {
            items = cornell_with_gas();
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
        lookat
    );

    camera.render(&world, background);
}
