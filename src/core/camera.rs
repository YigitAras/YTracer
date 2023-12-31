// TODO: Check the performance of rand::Rng. If it is a bottle neck change it.
use rand::Rng;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

use kdam::tqdm;
use rayon::prelude::*;

use crate::core::hittable::*;
use crate::geometry::ray::*;
use crate::geometry::vector3::*;
use crate::utils::*;

#[allow(dead_code)]
pub struct Camera {
    pub aspect_ratio: f64,      // Width Height ratio
    pub image_width: u64,       // Rendered image width in pixel count
    pub samples_per_pixel: u64, // Random samples for each pixel
    pub max_depth: u64,         // Maximum number of ray bounces

    pub vfov: f64,      // Vertical view angle (field of view)
    pub lookfrom: Vec3, // Point camera is looking from
    pub lookat: Vec3,   // Point camera is looking at
    pub vup: Vec3,      // Camera-relative up vector

    pub defocus_angle: f64, // Variation angle of rays through each pixel (?)
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: u64,    // Rendered image height
    sqrt_spp: u64,        // Square root of number of samples per pixel
    recip_sqrt_spp: f64,  // 1 / sqrt_spp
    center: Vec3,         // Camera center
    pixel00_loc: Vec3,    // Location of pixel 0,0
    pixel_delta_u: Vec3,  // Offset to pixel to the right
    pixel_delta_v: Vec3,  // Offset to pixel below
    u: Vec3,              // Camera basis vector
    v: Vec3,              // Camera basis vector
    w: Vec3,              // Camera basis vector
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn init(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
    ) -> Self
    {
        // Will be mostly this
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let defocus_angle = 0.0;
        // TODO: Maybe move this as an adjustable param
        let focus_dist = 10.0;

        let image_height = (image_width as f64 / aspect_ratio) as u64;

        let center = lookfrom;

        // Determine the viewport dims
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = (viewport_height * (image_width as f64 / image_height as f64)) as u64;

        let sqrt_spp = f64::sqrt(samples_per_pixel as f64) as u64;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        // Calculate the unit basis vecs for camera coord frame
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width as f64;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors to the next pixel
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
            sqrt_spp,
            recip_sqrt_spp,
        }
    }

    fn get_ray(&self, i: f64, j: f64, s_i: f64, s_j: f64) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);

        // This is just for adding anti aliasing (just jittering)
        //let pixel_sample = pixel_center + self.pixel_sample_square();

        // This is for stratified sampling in the pixel box
        let pixel_sample = pixel_center + self.pixel_sample_square(s_i, s_j);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self, s_i: f64, s_j: f64) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let mut rng = rand::thread_rng();
        let px = -0.5 + self.recip_sqrt_spp * (s_i + rng.gen::<f64>());
        let py = -0.5 + self.recip_sqrt_spp * (s_j + rng.gen::<f64>());
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }

    fn ray_color(r: Ray, background: Vec3, world: &dyn Hittable, depth: u64) -> Vec3 {
        // Depth limit reached don't accumulate any more light
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(r, 0.001, f64::MAX) {
            let emitted = hit.mat_ptr.emitted(&hit);
            if let Some((mut scattered, albedo, mut pdf)) = hit.mat_ptr.scatter(r, &hit) {
                // Light sampling -- hardcoded
                let on_light = Vec3::new(random_double(213.0, 343.0), 554.0, random_double(227.0, 332.0));
                let to_light = on_light - hit.p;
                let dist_sq = to_light.lenght_squared();

                if to_light.dot(hit.normal) < 0.0 {
                    return emitted;
                }

                let light_area = (343.0-213.0)*(332.0-227.0);
                let light_cos = f64::abs(to_light.y);

                if light_cos < 0.000001 {
                    return emitted;
                }

                pdf = dist_sq / (light_cos * light_area);
                scattered = Ray::new(hit.p, to_light);
                let scattering_pdf = hit.mat_ptr.scattering_pdf(r, &hit, scattered);
                emitted
                    + (Self::ray_color(scattered, background, world, depth - 1)
                        * scattering_pdf
                        * albedo)
                        / pdf
            } else {
                emitted
            }
        } else {
            // If not hit anything
            background
        }
    }

    pub fn render(&self, world: &dyn Hittable, background: Vec3) {
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
        let img_width = self.image_width;
        let img_height = self.image_height;
        let data = format!("P3\n{img_width} {img_height}\n255\n");

        file.write_all(data.as_bytes())
            .expect("Unable to write the header!");

        let rows: Vec<Vec<Vec3>> = tqdm!(0..self.image_height, animation = "fillup")
            .map(|j| {
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut col = Vec3::new(0.0, 0.0, 0.0);

                        for s_i in 0..self.sqrt_spp {
                            for s_j in 0..self.sqrt_spp {
                                let r = self.get_ray(i as f64, j as f64, s_i as f64, s_j as f64);
                                col += Self::ray_color(r, background, world, self.max_depth);
                            }
                        }

                        col /= self.samples_per_pixel as f64;
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
    }

}
