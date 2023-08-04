use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;

use std::sync::Arc;

type Link = Option<Box<BVHNode>>;

pub struct BVHNode {
    pub bbox: Aabb,
    pub left: Link,
    pub right: Link,
}

impl BVHNode {
    pub fn new(list: HittableList, start: u16, end: u16, time0: f64, tim1: f64) -> Self {
        let list_copy = list;
        BVHNode {
            bbox: Default::default(),
            left: None,
            right: None,
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
    fn bounding_box(self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = self.bbox;
        return true;
    }
}

#[inline]
fn box_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>, axis: i64) {
    let mut box_a: Aabb = Default::default();
    let mut box_b: Aabb = Default::default();

    if a.bounding_box(0.0, 0.0, &mut box_a) || b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVH_Node constructor!");
    }
}
