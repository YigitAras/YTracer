use std::f64::consts::PI;
use std::sync::Arc;

use crate::core::hittable::*;
use crate::geometry::onb::*;
use crate::geometry::ray::*;
use crate::geometry::vector3::*;
use crate::{texture::*, utils::*};

// Light reflection/refraction related utilities
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-f64::sqrt(f64::abs(1.0 - r_out_perp.lenght_squared())));
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Schlick approximation
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    // Returns Scattered Ray, Color and PDF
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3, f64)>;

    // As default objects shouldn't emit light
    fn emitted(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn from_color(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from_color(c)),
        }
    }
    #[allow(dead_code)]
    pub fn from_texture(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self {
            albedo: Arc::clone(&tex),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3, f64)> {
        let uvw = Onb::build_from_w(hit.normal);
        let scatter_dir = uvw.local(random_cosine_direction());

        let scattered = Ray::new(hit.p, Vec3::unit_vector(scatter_dir));
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        let pdf = uvw.w().dot(scattered.dir) / PI;
        Some((scattered, albedo, pdf))
    }
    fn scattering_pdf(&self, _r_in: Ray, hit: &HitRecord, scattered: Ray) -> f64 {
        let cos_theta = hit.normal.dot(Vec3::unit_vector(scattered.dir));
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}

/*
#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    #[allow(dead_code)]
    pub fn new(a: Vec3, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected =
            reflect(Vec3::unit_vector(r_in.dir), hit.normal) + random_in_unit_sphere() * self.fuzz;
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;

        if reflected.dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, refraction_ratio, cos_theta) = if ray.dir.dot(hit.normal) > 0.0 {
            let cos_theta = self.ir * ray.dir.dot(hit.normal) / ray.dir.length();
            (-hit.normal, self.ir, cos_theta)
        } else {
            let cos_theta = -ray.dir.dot(hit.normal) / ray.dir.length();
            (hit.normal, 1.0 / self.ir, cos_theta)
        };
        let unit_dir = Vec3::unit_vector(ray.dir);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            reflect(unit_dir, outward_normal)
        } else {
            refract(unit_dir, outward_normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.p, direction);
        Some((scattered, attenuation))
    }
}
*/

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(color: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::from_color(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: &HitRecord) -> Option<(Ray, Vec3, f64)> {
        // No reflection is done through the light
        None
    }
    fn emitted(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture + Sync + Send>,
}

impl Isotropic {
    pub fn from_color(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from_color(c)),
        }
    }
    #[allow(dead_code)]
    pub fn from_tex(texture: Arc<dyn Texture + Sync + Send>) -> Self {
        Self {
            albedo: Arc::clone(&texture),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3, f64)> {
        let scattered = Ray::new(hit.p, random_in_unit_sphere());
        let attenuation = self.albedo.value(hit.u, hit.v, hit.p);
        let pdf = 1.0 / (4.0 * PI);
        Some((scattered, attenuation, pdf))
    }

    fn scattering_pdf(&self, _r_in: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
