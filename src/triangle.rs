use crate::{vector3::*, hittable::*, aabb::*, ray::*};


const EPSILON: f64 = 0.0000001;

// TODO : Will most likely need color val/texture val for the triangles points
pub struct Triangle  {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Triangle {
    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self{
            a,b,c
        }
    } 
}

impl Hittable for Triangle {
    fn hit(&self, r: Ray, _: f64, _: f64) -> Option<HitRecord> {
        let vert0 = self.a;
        let vert1 = self.b;
        let vert2 = self.c;

        let edge1: Vec3;
        let edge2: Vec3;
        let h: Vec3;
        let s: Vec3;
        let q: Vec3;

        let a: f64;
        let f: f64;
        let u: f64;
        let v: f64;

        edge1 = vert1 - vert0;
        edge2 = vert2 - vert0;
        // h is the normal of the triangle
        h = r.dir.cross(edge2);
        a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            None  // Ray is parallel to the plane of triangle
        }

        f = 1.0 / a;
        s = r.orig - vert0;
        u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            None
        }

        q = s.cross(edge1);
        v = f * r.dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            None
        }

        // Now can compute t and find the intersection
        let t = f * edge2.dot(q);
        if t > EPSILON {
            let normal = Vec3::unit_vector(edge1.cross(edge2));
            let intersect_pt = r.at(t);
            // TODO: Set up the HitRecord here

            None
        } else {
            None
        }


        None
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        todo!()
    }
}