use std::sync::Arc;

use crate::{hittable::*, material::*, ray::*, aabb::*, vector3::*, texture::*};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Sync + Send>,
    phase_function: Arc<dyn Material + Sync + Send>,
    neg_inv_densiy: f64,
}

impl ConstantMedium {
    pub fn from_color(b: Arc<dyn Hittable + Sync + Send>, d: f64 ,c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_densiy: -1.0/d,
            phase_function: Arc::new(Isotropic::from_color(c))
        }
    }
    pub fn from_tex(b: Arc<dyn Hittable + Sync + Send>, d: f64 ,a: Arc<dyn Texture + Sync + Send>) -> Self {
        Self {
            boundary: b,
            neg_inv_densiy: -1.0/d,
            phase_function: Arc::new(Isotropic::from_tex(a))
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(r, f64::MIN, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t+0.0001, f64::MAX) {
                if hit1.t < t_min {hit1.t = t_min; }
                if hit2.t > t_max {hit2.t = t_max; }

                if hit1.t >= hit2.t {
                    return None;
                }
                // TODO: Continue here

                None
            } else {
                None
            }

        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}