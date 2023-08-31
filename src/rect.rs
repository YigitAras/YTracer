use std::sync::Arc;


use crate::{material::*, hittable::*, vector3::*};
use crate::aabb::Aabb;
use crate::ray::Ray;

pub enum Plane {
    YZ,
    ZX,
    XY
}

#[derive(Clone)]
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn Material + Sync + Send>,
}

// TODO: Turn this into AARect
impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;

        if t < t_min || t < t_max {
            return None;
        }

        let x = r.orig.x + t*r.dir.x;
        let y = r.orig.y + t*r.dir.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }


        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = r.at(t);
        let normal = Vec3::new(0.0,0.0,0.1);

        Some(HitRecord { p, normal, t, u, v, mat_ptr: Arc::clone(&self.mp) })
    }
    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        // Add a little bit of padding
        *output_box = Aabb{
            minimum: Vec3::new(self.x0,self.y0, self.k-0.0001),
            maximum: Vec3::new(self.x1, self.y1, self.k+0.0001)};
        true
    }
}
