use std::sync::Arc;

use crate::{hittable::*, vector3::*,ray::*,aabb::*,utils::*};

/*
 *  Instead of moving the object, just move the ray in opposite direction
 *  And then do the tests. Handle Hit Records and Bounding Boxes accordingly.
 */
pub struct Translate {
    obj_ptr: Arc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(obj_ptr: Arc<dyn Hittable>, offset: Vec3) ->Self {
        Self {
            obj_ptr, offset
        }
    }
}


impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig-self.offset, r.dir);

        if let Some(mut hit) = self.obj_ptr.hit(moved_r, t_min, t_max) {
            hit.p += self.offset;
            hit.set_face_normal(moved_r, hit.normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.obj_ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = Aabb{
            minimum: output_box.minimum + self.offset,
            maximum: output_box.maximum + self.offset
        };
        true
    }
}

pub struct YRotate {
    sin_theta: f64,
    cos_theta: f64,
    obj_ptr: Arc<dyn Hittable>,
    bbox: Aabb
}

impl YRotate {
    pub fn new(obj_ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let rads = degrees_to_radians(angle);
        let sin_theta = f64::sin(rads);
        let cos_theta = f64::cos(rads);
        let mut bbox = Default::default();
        // TODO: Change the API to return Optional<Aabb>
        let _ = obj_ptr.bounding_box(0.0,0.0,&mut bbox);

        let mut min = Vec3::new(f64::MAX,f64::MAX, f64::MAX);
        let mut max = Vec3::new(f64::MIN,f64::MIN,f64::MIN);

        for  i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x +  (1-i) as f64 * bbox.minimum.x;
                    let y = j as f64 * bbox.maximum.y +  (1-j) as f64 * bbox.minimum.y;
                    let z = k as f64 * bbox.maximum.z +  (1-k) as f64 * bbox.minimum.z;

                    let newx =  cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        bbox = Aabb {minimum: min, maximum: max};

        Self {
            sin_theta,
            cos_theta,
            obj_ptr,
            bbox
        }

    }
}

impl Hittable for YRotate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        direction[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        direction[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_r = Ray::new(origin, direction);

        if let Some(mut hit) = self.obj_ptr.hit(rotated_r, t_min, t_max) {
            let mut p = hit.p;
            let mut normal = hit.normal;
            p[0] =  self.cos_theta*hit.p[0] + self.sin_theta*hit.p[2];
            p[2] = -self.sin_theta*hit.p[0] + self.cos_theta*hit.p[2];

            normal[0] =  self.cos_theta*hit.normal[0] + self.sin_theta*hit.normal[2];
            normal[2] = -self.sin_theta*hit.normal[0] + self.cos_theta*hit.normal[2];

            hit.p = p;
            hit.set_face_normal(rotated_r, normal);

            Some(hit)
        } else {
            None
        }
    }
    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}