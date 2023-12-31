use std::sync::Arc;

use crate::accelerators::aabb::*;
use crate::core::hittable::*;
use crate::core::hittable_list::*;
use crate::geometry::ray::*;
use crate::geometry::vector3::*;
use crate::material::*;

use crate::primitives::triangle::*;
use crate::primitives::quad::*;

#[derive(Clone)]
pub enum Plane {
    YZ,
    XZ,
    XY,
}

#[derive(Clone)]
pub struct AARect {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    mp: Arc<dyn Material + Sync + Send>,
}

// TODO: Turn this into AARect
impl AARect {
    pub fn new(
        plane: Plane,
        a0: f64,
        a1: f64,
        b0: f64,
        b1: f64,
        k: f64,
        mp: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            mp,
        }
    }
}

impl Hittable for AARect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis, first_axis, second_axis) = match &self.plane {
            Plane::YZ => (0usize, 1usize, 2usize),
            Plane::XZ => (1usize, 0usize, 2usize),
            Plane::XY => (2usize, 0usize, 1usize),
        };
        let t = (self.k - r.orig[k_axis]) / r.dir[k_axis];

        if t < t_min || t > t_max {
            return None;
        }

        let a = r.orig[first_axis] + t * r.dir[first_axis];
        let b = r.orig[second_axis] + t * r.dir[second_axis];

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let p = r.at(t);
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        normal[k_axis] = 1.0;
        let mut hit_rect = HitRecord::new(p, normal, t, u, v, Arc::clone(&self.mp));
        hit_rect.set_face_normal(r, normal);

        Some(hit_rect)
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        // Add a little bit of padding
        match self.plane {
            Plane::YZ => {
                *output_box = Aabb {
                    minimum: Vec3::new(self.k - 0.0001, self.a0, self.b0),
                    maximum: Vec3::new(self.k + 0.0001, self.a1, self.b1),
                };
            }
            Plane::XZ => {
                *output_box = Aabb {
                    minimum: Vec3::new(self.a0, self.k - 0.0001, self.b0),
                    maximum: Vec3::new(self.a1, self.k + 0.0001, self.b1),
                };
            }
            Plane::XY => {
                *output_box = Aabb {
                    minimum: Vec3::new(self.a0, self.b0, self.k - 0.0001),
                    maximum: Vec3::new(self.a1, self.b1, self.k + 0.0001),
                };
            }
        }

        true
    }
}

// Box object created via Rectangle objects
#[derive(Clone)]
pub struct Box {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl Box {
    pub fn new_triangles(p0: Vec3, p1: Vec3, mat_ptr: Arc<dyn Material + Sync + Send>) -> Self {
        let mut sides: HittableList = Default::default();

        // Triangulation code for less code repeat
        #[inline]
        fn triangulate(
            list: &mut HittableList,
            t_num: usize,
            a: (f64, f64),
            b: (f64, f64),
            k: f64,
            plane: Plane,
            mat_ptr: Arc<dyn Material + Sync + Send>,
        ) {
            let (k_axis, first_axis, second_axis) = match plane {
                Plane::YZ => (0usize, 1usize, 2usize),
                Plane::XZ => (1usize, 0usize, 2usize),
                Plane::XY => (2usize, 0usize, 1usize),
            };
            let mut x = Vec3::new(0.0, 0.0, 0.0);
            let mut y = Vec3::new(0.0, 0.0, 0.0);
            let mut z = Vec3::new(0.0, 0.0, 0.0);
            match t_num {
                1 => {
                    x[first_axis] = a.0;
                    x[second_axis] = b.0;
                    x[k_axis] = k;
                    y[first_axis] = a.1;
                    y[second_axis] = b.0;
                    y[k_axis] = k;
                    z[first_axis] = a.0;
                    z[second_axis] = b.1;
                    z[k_axis] = k;
                }
                2 => {
                    x[first_axis] = a.1;
                    x[second_axis] = b.1;
                    x[k_axis] = k;
                    y[first_axis] = a.0;
                    y[second_axis] = b.1;
                    y[k_axis] = k;
                    z[first_axis] = a.1;
                    z[second_axis] = b.0;
                    z[k_axis] = k;
                }
                _ => {
                    panic!["Unexpected triangulation at Box!"]
                }
            }
            list.add(Arc::new(Triangle::from_points(
                x,
                y,
                z,
                Arc::clone(&mat_ptr),
            )));
        }

        // XY Plane sides
        // Side 1
        triangulate(
            &mut sides,
            1,
            (p0.x, p1.x),
            (p0.y, p1.y),
            p1.z,
            Plane::XY,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.x, p1.x),
            (p0.y, p1.y),
            p1.z,
            Plane::XY,
            Arc::clone(&mat_ptr),
        );
        // Side 2
        triangulate(
            &mut sides,
            1,
            (p0.x, p1.x),
            (p0.y, p1.y),
            p0.z,
            Plane::XY,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.x, p1.x),
            (p0.y, p1.y),
            p0.z,
            Plane::XY,
            Arc::clone(&mat_ptr),
        );

        // XZ Plane sides
        // Side 1
        triangulate(
            &mut sides,
            1,
            (p0.x, p1.x),
            (p0.z, p1.z),
            p1.y,
            Plane::XZ,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.x, p1.x),
            (p0.z, p1.z),
            p1.y,
            Plane::XZ,
            Arc::clone(&mat_ptr),
        );
        // Side 2
        triangulate(
            &mut sides,
            1,
            (p0.x, p1.x),
            (p0.z, p1.z),
            p0.y,
            Plane::XZ,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.x, p1.x),
            (p0.z, p1.z),
            p0.y,
            Plane::XZ,
            Arc::clone(&mat_ptr),
        );

        // YZ Plane Sides
        // Side 1
        triangulate(
            &mut sides,
            1,
            (p0.y, p1.y),
            (p0.z, p1.z),
            p1.x,
            Plane::YZ,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.y, p1.y),
            (p0.z, p1.z),
            p1.x,
            Plane::YZ,
            Arc::clone(&mat_ptr),
        );
        // Side 2
        triangulate(
            &mut sides,
            1,
            (p0.y, p1.y),
            (p0.z, p1.z),
            p0.x,
            Plane::YZ,
            Arc::clone(&mat_ptr),
        );
        triangulate(
            &mut sides,
            2,
            (p0.y, p1.y),
            (p0.z, p1.z),
            p0.x,
            Plane::YZ,
            Arc::clone(&mat_ptr),
        );

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
    pub fn new(p0: Vec3, p1: Vec3, mat_ptr: Arc<dyn Material + Sync + Send>) -> Self {
        let mut sides: HittableList = Default::default();

        // XY Plane sides
        sides.add(Arc::new(AARect::new(
            Plane::XY,
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            Arc::clone(&mat_ptr),
        )));

        sides.add(Arc::new(AARect::new(
            Plane::XY,
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            Arc::clone(&mat_ptr),
        )));

        // XZ Plane sides
        sides.add(Arc::new(AARect::new(
            Plane::XZ,
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            Arc::clone(&mat_ptr),
        )));
        sides.add(Arc::new(AARect::new(
            Plane::XZ,
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            Arc::clone(&mat_ptr),
        )));
        // YZ Plane Sides
        sides.add(Arc::new(AARect::new(
            Plane::YZ,
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            Arc::clone(&mat_ptr),
        )));
        sides.add(Arc::new(AARect::new(
            Plane::YZ,
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            Arc::clone(&mat_ptr),
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
    pub fn from_quads(p0: Vec3, p1: Vec3, mat_ptr: Arc<dyn Material + Sync + Send>) -> Self {
        let mut sides: HittableList = Default::default();
        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Vec3::new(f64::min(p0.x, p1.x),
                                  f64::min(p0.y, p1.y),
                                  f64::min(p0.z, p1.z));
        let max = Vec3::new(f64::max(p0.x, p1.x),
                                  f64::max(p0.y, p1.y),
                                  f64::max(p0.z, p1.z));

        let dx = Vec3::new(max.x- min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Arc::new(Quad::new(
            Vec3::new(min.x, min.y, max.z),  dx,  dy, Arc::clone(&mat_ptr)
        ))); // front
        sides.add(Arc::new(Quad::new(
            Vec3::new(max.x, min.y, max.z), -dz,  dy, Arc::clone(&mat_ptr)
        ))); // right
        sides.add(Arc::new(Quad::new(
            Vec3::new(max.x, min.y, min.z), -dx,  dy, Arc::clone(&mat_ptr)
        ))); // back
        sides.add(Arc::new(Quad::new(
            Vec3::new(min.x, min.y, max.z),  dz,  dy, Arc::clone(&mat_ptr)
        ))); // left
        sides.add(Arc::new(Quad::new(
            Vec3::new(min.x, min.y, max.z),  dx, -dz, Arc::clone(&mat_ptr)
        ))); // top
        sides.add(Arc::new(Quad::new(
            Vec3::new(min.x, min.y, max.z),  dx,  dz, Arc::clone(&mat_ptr)
        ))); // bottom

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for Box {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Just relay it to the Hittable list sides
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: self.box_min,
            maximum: self.box_max,
        };
        true
    }
}
