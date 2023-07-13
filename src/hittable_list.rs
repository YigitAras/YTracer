use crate::hittable::*;

pub struct HittableList {

}


impl Hittable for hittable_list {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        true
    }
}