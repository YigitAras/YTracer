use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;

use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

enum BVHNode {
    Branch {
        left: Arc<BVHNode>,
        right: Arc<BVHNode>,
    },
    Left(Arc<dyn Hittable>),
}

pub struct BVH {
    bbox: Aabb,
    tree: BVHNode,
}

impl BVH {
    pub fn new(
        objects_copy: &mut HittableList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        //let mut objects_copy = list.clone();
        let mut rng = rand::thread_rng();
        let axis: u8 = rng.gen_range(0..=2);

        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        BVH {
            bbox: Default::default(),
            tree: None,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min..t_max) {
            match &self.tree {
                BVHNode::Left(leaf) => None,
                BVHNode::Branch {left, right} => None,
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        true
    }
}

#[inline]
fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a: Aabb = Default::default();
    let mut box_b: Aabb = Default::default();

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVH_Node comparator!");
    }

    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
}

#[inline]
fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

#[inline]
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

#[inline]
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
