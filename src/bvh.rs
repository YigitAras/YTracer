use rayon::prelude::IndexedParallelIterator;

use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;

use std::sync::Arc;
use rand::Rng;

type Link = Option<Arc<dyn Hittable>>;

pub struct BVHNode {
    pub bbox: Aabb,
    pub left: Link,
    pub right: Link,
}

impl BVHNode {
    pub fn new(list: HittableList, start: usize, end: usize, time0: f64, tim1: f64) -> Self {
        
        let objects_copy = list.clone();
        let mut rng = rand::thread_rng();
        let axis: u8 = rng.gen_range(0..=2);

        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        let mut left_n: Option<Arc<dyn Hittable>> = None;
        let mut right_n: Option<Arc<dyn Hittable>> = None;
        if object_span == 1 {
            left_n = Some(objects_copy[start]);
            right_n = Some(objects_copy[start]);
        } else if object_span == 2 {
            if comparator(objects_copy[start], objects_copy[start+1]) {
                left_n = Some(objects_copy[start]);
                right_n = Some(objects_copy[start+1]);
            } else {
                left_n = Some(objects_copy[start+1]);
                right_n = Some(objects_copy[start]);
            }
        } else {
            /* TODO: Sort the iter+start - iter+end of the vector */
        }
        BVHNode {
            bbox: Default::default(),
            left: left_n,
            right: right_n,
        }
        
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min..t_max) {
            return None;
        }
        // Check left and right hit
        let tmp_hit = match &self.left {
            None => None,
            Some(node) => node.hit(r, t_min, t_max),
        };
        let final_hit = match &self.right {
            None => tmp_hit,
            Some(node) => {
                let mut furthest = t_max;
                if let Some(hit) = tmp_hit {
                    furthest = hit.t;
                }
                node.hit(r, t_min, furthest)
            }
        };
        final_hit
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        return true;
    }
}

#[inline]
fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> bool {
    let mut box_a: Aabb = Default::default();
    let mut box_b: Aabb = Default::default();

    if a.bounding_box(0.0, 0.0, &mut box_a) || b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVH_Node constructor!");
    }

    box_a.min()[axis] < box_b.min()[axis]
}

#[inline]
fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 0)
} 

#[inline]
fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 1)
}

#[inline]
fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 2)
}