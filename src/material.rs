use crate::utils::*;
use crate::hittable::*;
use crate::vector3::*;
use crate::ray::*;

use rand::Rng;
use dyn_clone::*;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p =  Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0  - unit;
        if p.lenght_squared() < 1.0 {
            return p
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3{
    v -  n * v.dot(n) * 2.0
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = (uv+n*cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-f64::sqrt(f64::abs(1.0 - r_out_perp.lenght_squared())));
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> bool {
    // Schlick approximation
    let mut rng = rand::thread_rng();
    let r0 = ((1.0-ref_idx)/(1.0+ref_idx)).powi(2);
    (r0 + (1.0-r0) * (1.0-cosine).powi(5)) > rng.gen_range(0.0..1.0)
}

pub trait Material: DynClone  {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f64
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 {
                f
            } else {
                1.0
            }

        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = hit.normal + random_in_unit_sphere();
        
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }
        
        let scattered = Ray::new(hit.p, scatter_dir);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(Vec3::unit_vector(r_in.dir), hit.normal) + random_in_unit_sphere()* self.fuzz;
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;
        
        if reflected.dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0,1.0,1.0);
        let refraction_ratio = 1.0/self.ir;

        let unit_dir = Vec3::unit_vector(r_in.dir);
        
        let cos_theta = f64::min(-unit_dir.dot(hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0-cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, refraction_ratio){
            direction = reflect(unit_dir, hit.normal);
        } else {
            direction = refract(unit_dir, hit.normal, refraction_ratio);
        }
        
        let scattered = Ray::new(hit.p, direction);

        Some((scattered, attenuation))
    }
}
