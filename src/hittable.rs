use crate::aabb::*;
use crate::material::*;
use crate::ray::*;
use crate::vector3::*;

use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material + Sync + Send>,
    pub t: f64,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f64,
        mat_ptr: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            p: point,
            normal,
            mat_ptr,
            t,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // TODO: Rustify this part, currently it is C-like
    fn bounding_box(self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}
