use std::sync::Arc;

use crate::core::hittable::*;
use crate::core::hittable_list::*;
use crate::geometry::instance::*;
use crate::geometry::vector3::*;
use crate::primitives::{mesh::*, rect::*, sphere::*};
use crate::{constant_medium::*, material::*, texture::*};
pub struct Scene {
    // Can add env related things here
}

impl Scene {
    pub fn checker_world() -> HittableList {
        let mut world: HittableList = Default::default();
        let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::from_color(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ));
        let mat_checker: Arc<dyn Material + Sync + Send> =
            Arc::new(Lambertian::from_texture(checker));
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

    pub fn two_perlin_spheres() -> HittableList {
        let mut world: HittableList = Default::default();
        let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
        let mat_perlin: Arc<dyn Material + Sync + Send> =
            Arc::new(Lambertian::from_texture(pertext));
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

    pub fn earth_scene() -> HittableList {
        let mut world: HittableList = Default::default();
        let image = image::open("static/moon.jpg")
            .expect("image not found")
            .to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        let texture: Arc<dyn Texture + Sync + Send> =
            Arc::new(ImageTexture::new(data, width as u64, height as u64));
        let mat_world: Arc<dyn Material + Sync + Send> =
            Arc::new(Lambertian::from_texture(texture));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            2.0,
            Arc::clone(&mat_world),
        )));

        world
    }

    pub fn simple_light() -> HittableList {
        let mut world: HittableList = Default::default();
        let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
        let mat_perlin: Arc<dyn Material + Sync + Send> =
            Arc::new(Lambertian::from_texture(pertext));
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

    pub fn cornell_box() -> HittableList {
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
        let mut box1: Arc<dyn Hittable> = Arc::new(Box::new_triangles(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            Arc::clone(&white),
        ));

        box1 = Arc::new(YRotate::new(box1, 15.0));
        box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
        world.add(box1);

        let mut box2: Arc<dyn Hittable> = Arc::new(Box::new_triangles(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            Arc::clone(&white),
        ));

        box2 = Arc::new(YRotate::new(box2, -18.0));
        box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
        world.add(box2);

        world
    }

    pub fn cornell_with_gas() -> HittableList {
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

    pub fn cornell_with_mesh(big_mesh: bool) -> HittableList {
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

        // Hand-waivy location core
        let mut first_loc = (Vec3::new(0.0, 0.0, 0.0) + Vec3::new(165.0, 165.0, 165.0)) / 2.0;
        first_loc += Vec3::new(280.0, 165.0 / 2.0 - 35.0, 220.0);

        let mut second_loc = (Vec3::new(0.0, 0.0, 0.0) + Vec3::new(165.0, 165.0, 165.0)) / 2.0;
        second_loc += Vec3::new(130.0, 0.0, 65.0);
        second_loc += Vec3::new(-70.0, 30.0, -40.0);

        let mut bunny_mesh: Arc<dyn Hittable> = Arc::new(Mesh::new(
            "static/stanford_bunny.obj",
            Arc::clone(&white),
            Vec3::new(1000.0, 1000.0, 1000.0),
        ));

        bunny_mesh = Arc::new(YRotate::new(bunny_mesh, 210.0));
        bunny_mesh = Arc::new(Translate::new(bunny_mesh, first_loc));

        let mut second_mesh: Arc<dyn Hittable> = Arc::new(Mesh::new(
            "static/monkey.obj",
            Arc::clone(&white),
            Vec3::new(85.0, 85.0, 85.0),
        ));

        // Just read the Ajax mesh, although it is quite big...
        if big_mesh {
            second_mesh = Arc::new(Mesh::new(
                "static/ajax.obj",
                Arc::clone(&white),
                Vec3::new(100.0, 100.0, 100.0),
            ));
        }

        second_mesh = Arc::new(YRotate::new(second_mesh, 150.0));
        second_mesh = Arc::new(Translate::new(second_mesh, second_loc));

        world.add(bunny_mesh);
        world.add(second_mesh);

        let mut box2: Arc<dyn Hittable> = Arc::new(Box::new_triangles(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            Arc::clone(&white),
        ));

        box2 = Arc::new(YRotate::new(box2, -18.0));
        box2 = Arc::new(Translate::new(box2, Vec3::new(320.0, 0.0, 240.0)));
        world.add(box2);

        world
    }
}
