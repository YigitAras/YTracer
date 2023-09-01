use std::sync::Arc;


use crate::{material::*, hittable::*, vector3::*};
use crate::aabb::Aabb;
use crate::ray::Ray;

#[derive(Clone)]
pub enum Plane {
    YZ,
    ZX,
    XY
}

#[derive(Clone)]
pub struct AARect {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    mp: Arc<dyn Material + Sync + Send>,
}

// TODO: Turn this into AARect
impl AARect {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64, mp: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            mp,
        }
    }
}

impl Hittable for AARect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis, first_axis, second_axis) = match &self.plane {
            Plane::YZ => (0usize, 1usize, 2usize),
            Plane::ZX => (1usize, 2usize, 0usize),
            Plane::XY => (2usize, 0usize, 1usize)
        };
        let t = (self.k - r.orig[k_axis]) / r.dir[k_axis];


        if t < t_min || t < t_max {
            return None;
        }

        let a = r.orig[first_axis] + t * r.dir[first_axis];
        let b = r.orig[second_axis] + t * r.dir[second_axis];

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }


        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let p = r.at(t);
        let mut normal = Vec3::new(0.0,0.0,0.0);
        normal[k_axis] = 1.0;

        Some(HitRecord { p, normal, t, u, v, mat_ptr: Arc::clone(&self.mp) })
    }
    // TODO: Does not look right. Adjust the epsilon side depending on the plane
    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        // Add a little bit of padding
        *output_box = Aabb{
            minimum: Vec3::new(self.a0,self.b0, self.k-0.0001),
            maximum: Vec3::new(self.a1, self.b1, self.k+0.0001)};
        true
    }
}
