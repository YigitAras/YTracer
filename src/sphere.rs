use crate::hittable::*;
use crate::vector3::*;
use crate::ray::*;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64) ->Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - center;
        let a  = r.dir.lenght_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.lenght_squared() - radius*radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-helf_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            } 
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        outward_normal = (rec.p - center) / radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}