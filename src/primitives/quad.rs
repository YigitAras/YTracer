use std::sync::Arc;

use crate::accelerators::aabb::Aabb;
use crate::core::hittable::{HitRecord, Hittable};
use crate::geometry::{ray::*, vector3::*};
use crate::material::*;

pub struct Quad {
    q: Vec3,    // Lower left corner of quad
    u: Vec3,    // Direction basis vector 1
    v: Vec3,    // Direction basis vector 2
    mat_ptr: Arc<dyn Material + Sync + Send>,
    bbox: Aabb,
    normal: Vec3,
    w: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat_ptr: Arc<dyn Material + Sync + Send>) -> Self {
        let bbox = Quad::set_bounding_box(q,u,v);
        let n = u.cross(v);
        let normal = Vec3::unit_vector(n);
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self {
            q,
            u,
            v,
            mat_ptr,
            bbox,
            normal,
            w,
            d
        }
    }
    #[inline]
    fn set_bounding_box(q:Vec3, u: Vec3, v: Vec3) -> Aabb {
        // Padding to reduce
        Aabb{
            minimum: q,
            maximum: q+u+v
        }.pad()
    }
    #[inline]
    fn is_interior(a: f64, b: f64) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        !(!(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b))
    }
}


impl Hittable for Quad {
    #[inline]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(r.dir);

        // Ray is parallel to the plane
        if f64::abs(denom) < 1e-8 {
            return None;
        }

        // False if hit point t is outside the ray interval
        let t = (self.d - self.normal.dot(r.orig)) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        // Determine hit point using plane coords
        let intersect = r.at(t);
        let planaer_hitpt_vec = intersect - self.q;
        // Alpha and  Beta parameters of the plane
        let alpha = self.w.dot(planaer_hitpt_vec.cross(self.v));
        let beta  = self.w.dot(self.u.cross(planaer_hitpt_vec));

        if !Quad::is_interior(alpha, beta) {
            return None;
        }

        let mut hit = HitRecord {
            t,
            p: intersect,
            mat_ptr: Arc::clone(&self.mat_ptr),
            front_face: true,
            normal: self.normal,
            u: alpha,
            v: beta,
        };
        hit.set_face_normal(r, hit.normal);


        Some(hit)
    }
    #[inline]
    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }

}