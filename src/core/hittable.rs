use crate::accelerators::aabb::*;
use crate::geometry::ray::*;
use crate::geometry::vector3::*;
use crate::material::*;

use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material + Sync + Send>,
    pub u: f64,
    pub v: f64, // U and V for texture values
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        mat_ptr: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            p: point,
            normal,
            mat_ptr,
            u,
            v,
            t,
            front_face: true, // Placeholder
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // TODO: Rustify this part, currently it is C-like
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}
