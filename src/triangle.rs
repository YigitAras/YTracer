use std::sync::Arc;

use crate::{aabb::*, hittable::*, material::*, ray::*, vector3::*};

const EPSILON: f64 = 0.0000001;

// TODO : Normal is already being interpolated
// TODO : Make sure the color of mat's is also interpolated...?
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    mat_ptr: Arc<dyn Material + Sync + Send>,
}

impl Triangle {
    #[allow(dead_code)]
    pub fn from_points(
        a: Vec3,
        b: Vec3,
        c: Vec3,
        mat_ptr: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self { a, b, c, mat_ptr }
    }
}

impl Hittable for Triangle {
    // U for A, V for B and W for c
    // Where 1-U+V = W ?
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vert0 = self.a;
        let vert1 = self.b;
        let vert2 = self.c;

        let edge1: Vec3 = vert1 - vert0;
        let edge2: Vec3 = vert2 - vert0;
        let pvec: Vec3 = r.dir.cross(edge2);
        let det: f64 = edge1.dot(pvec);

        if f64::abs(det) < EPSILON {
            return None; // Ray is parallel to the plane of triangle
        }

        let inv_Det: f64 = 1.0 / det;
        let tvec: Vec3 = r.orig - vert0;
        let u: f64 = inv_Det * tvec.dot(pvec);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let qvec: Vec3 = tvec.cross(edge1);
        let v: f64 = inv_Det * r.dir.dot(qvec);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Now can compute t and find the intersection
        let t = inv_Det * edge2.dot(qvec);

        if t < t_min || t > t_max {
            return None;
        }

        if t > EPSILON {
            // TODO: Can simply calculate the normal of triangle
            let normal = Vec3::unit_vector(edge1.cross(edge2));
            // U for A, V for B and W for c
            // TODO: Can read normals from the file
            // Linearly interp the normals
            // let normal =
            //    Vec3::unit_vector(self.a_norm * u + self.b_norm * v + self.c_norm * (1.0 - u - v));

            let p = r.at(t);

            let mut hit = HitRecord::new(p, normal, t, u, v, Arc::clone(&self.mat_ptr));
            // Set the face of the triangle
            hit.set_face_normal(r, normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let a = self.a;
        let b = self.b;
        let c = self.c;

        let minimum = Vec3::new(
            f64::min(a.x, f64::min(b.x, c.x)) - 0.0001,
            f64::min(a.y, f64::min(b.y, c.y)) - 0.0001,
            f64::min(a.z, f64::min(b.z, c.z)) - 0.0001,
        );

        let maximum = Vec3::new(
            f64::max(a.x, f64::max(b.x, c.x)) + 0.0001,
            f64::max(a.y, f64::max(b.y, c.y)) + 0.0001,
            f64::max(a.z, f64::max(b.z, c.z)) + 0.0001,
        );

        *output_box = Aabb { minimum, maximum };
        true
    }
}
