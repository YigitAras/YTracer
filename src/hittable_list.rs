use std::sync::Arc;

use std::ops::{Index, IndexMut};

use crate::aabb::*;
use crate::hittable::*;
use crate::ray::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
    // TODO: Rustify this part, currently it is C-like
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box: Aabb = Default::default();
        let mut first_box = true;

        for obj in self.objects.iter() {
            if !obj.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                output_box.surrounding_box(temp_box)
            };
            first_box = false;
        }

        true
    }
}

impl Index<usize> for HittableList {
    type Output = Arc<dyn Hittable>;
    #[inline]
    fn index<'a>(&'a self, i: usize) -> &'a Arc<dyn Hittable> {
        &self.objects[i]
    }
}

impl IndexMut<usize> for HittableList {
    #[inline]
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Arc<dyn Hittable> {
        &mut self.objects[i]
    }
}
