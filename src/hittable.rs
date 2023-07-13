use crate::ray::*;
use crate::vector3::*;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p: point,
            normal: normal,
            t: t,
            front_face: front_face
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
        self.front_face = r.dir.dot(outward_normal) < 0.0;
    }
}

trait Hittable{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}