use crate::ray::*;
use crate::vector3::*;
use crate::material::*;


pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Box<dyn Material>,
    pub t: f64,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, mat_ptr: Box<dyn Material>) -> Self {
        Self {
            p: point,
            normal: normal,
            mat_ptr: mat_ptr,
            t: t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
