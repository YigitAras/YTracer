use rand::Rng;
use std::sync::Arc;

use crate::{aabb::*, hittable::*, material::*, ray::*, texture::*, vector3::*};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Sync + Send>,
    phase_function: Arc<dyn Material + Sync + Send>,
    neg_inv_densiy: f64,
}

impl ConstantMedium {
    pub fn from_color(b: Arc<dyn Hittable + Sync + Send>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_densiy: -1.0 / d,
            phase_function: Arc::new(Isotropic::from_color(c)),
        }
    }
    #[allow(dead_code)]
    pub fn from_tex(
        b: Arc<dyn Hittable + Sync + Send>,
        d: f64,
        a: Arc<dyn Texture + Sync + Send>,
    ) -> Self {
        Self {
            boundary: b,
            neg_inv_densiy: -1.0 / d,
            phase_function: Arc::new(Isotropic::from_tex(a)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(r, f64::MIN, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, f64::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let ray_len = r.dir.length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_len;
                let hit_distance = self.neg_inv_densiy * rng.gen::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = hit1.t + hit_distance / ray_len;
                Some(HitRecord {
                    t,
                    u: 0.0,
                    v: 0.0,
                    p: r.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
                    front_face: true,                 // Also arbitrary
                    mat_ptr: Arc::clone(&self.phase_function),
                })
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
