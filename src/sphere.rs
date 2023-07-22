use crate::hittable::*;
use crate::vector3::*;
use crate::ray::*;
use crate::material::*;

use std::rc::Rc;


#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Rc<dyn Material>
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, mat_ptr: Rc<dyn Material>) ->Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: mat_ptr
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a  = r.dir.lenght_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.lenght_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
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


        return Some(HitRecord { p: p_temp, normal: normal, t: t_temp, mat_ptr: dyn_clone::clone_box(&*self.mat_ptr)});
    }
}