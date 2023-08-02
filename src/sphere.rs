use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vector3::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, mat_ptr: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.lenght_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.lenght_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t_temp = root;
        let p_temp = r.at(t_temp);
        let normal = (p_temp - self.center) / self.radius;

        Some(HitRecord::new(
            p_temp,
            normal,
            t_temp,
            Arc::clone(&self.mat_ptr),
        ))
    }

    // TODO: Rustify this part, currently it is C-like
    fn bounding_box(&mut self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        return true;
    }
}
