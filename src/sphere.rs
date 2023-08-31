use std::sync::Arc;

use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vector3::*;

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
    #[inline]
    pub fn get_uv(p: Vec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1
        // v: returned value [0,1] of angle from Y=-1 to Y=+1
        // <1 0 0> yields <0.50 0.50> <-1 0 0> yields <0.00 0.50>
        // <0 1 0> yields <0.50 1.00> <0 -1 0> yields <0.50 0.00>
        // <0 0 1> yields <0.25 0.50> <0 0 -1> yields <0.75 0.50>

        // Compute (theta,phi) in spherical coords
        // Then map to texture coords U and V
        // u = phi/2pi v = theta/pi
        // y = - cos(theta)
        // x = -cos(phi)sin(theta)
        // z = sin(phi)sin(theta)

        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + std::f64::consts::PI;

        // (U , V)
        (
            phi / (2.0 * std::f64::consts::PI),
            theta / std::f64::consts::PI,
        )
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
        let sqrt_disc = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_disc) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_disc) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t_temp = root;
        let p_temp = r.at(t_temp);
        let normal = (p_temp - self.center) / self.radius;
        let (u, v) = Sphere::get_uv(normal);

        Some(HitRecord::new(
            p_temp,
            normal,
            t_temp,
            u,
            v,
            Arc::clone(&self.mat_ptr),
        ))
    }

    // TODO: Rustify this part, currently it is C-like
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        true
    }
}
