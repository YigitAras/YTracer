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

pub trait Material: DynClone  {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
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
        let reflected = reflect(Vec3::unit_vector(r_in.dir), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;
        
        if reflected.dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
