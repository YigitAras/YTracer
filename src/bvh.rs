use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;

use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

enum BVHNode {
    Branch { left: Arc<BVH>, right: Arc<BVH> },
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
        let axis: usize = rng.gen_range(0..=2);
        let object_span = end - start;
        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        // Sort the objects
        objects_copy.objects[start..end].sort_unstable_by(comparator);

        let len = objects_copy.objects[start..end].len();

        match len {
            0 => panic!["No elements in scene!"],
            1 => {
                let leaf = Arc::clone(&objects_copy.objects[start]);
                let mut tmp_bbox: Aabb = Default::default();
                if leaf.bounding_box(time0, time1, &mut tmp_bbox) {
                    BVH {
                        tree: BVHNode::Left(leaf),
                        bbox: tmp_bbox,
                    }
                } else {
                    panic!["No bounding box in BVH Node"]
                }
            }
            _ => {
                let mid = start + object_span / 2;
                let left = BVH::new(objects_copy, start, mid, time0, time1);
                let right = BVH::new(objects_copy, mid, end, time0, time1);
                let tmp_bbox = left.bbox.surrounding_box(right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Arc::new(left),
                        right: Arc::new(right),
                    },
                    bbox: tmp_bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min..t_max) {
            match &self.tree {
                BVHNode::Left(_leaf) => None,
                BVHNode::Branch { left: _, right: _ } => None,
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
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
