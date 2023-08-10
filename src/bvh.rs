use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;

use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

type Link = Option<Arc<dyn Hittable>>;

pub struct BVHNode {
    pub bbox: Aabb,
    pub left: Link,
    pub right: Link,
}

impl BVHNode {
    fn axis_range(objs: &HittableList, axis: usize) -> f64 {
        let range = objs
            .objects
            .iter()
            .fold(std::f64::MAX..std::f64::MIN, |range, o| {
                let mut bb: Aabb = Default::default();
                o.bounding_box(0.0, 0.0, &mut bb);
                let min = bb.minimum[axis].min(bb.maximum[axis]);
                let max = bb.minimum[axis].max(bb.maximum[axis]);
                range.start.min(min)..range.end.max(max)
            });
        range.end - range.start
    }

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
        let left_n: Option<Arc<dyn Hittable>>;
        let right_n: Option<Arc<dyn Hittable>>;
        if object_span == 1 {
            left_n = Some(Arc::clone(&objects_copy[start]));
            right_n = Some(Arc::clone(&objects_copy[start]));
        } else if object_span == 2 {
            if comparator(&objects_copy[start], &objects_copy[start + 1]) == Ordering::Less {
                left_n = Some(Arc::clone(&objects_copy[start]));
                right_n = Some(Arc::clone(&objects_copy[start + 1]));
            } else {
                left_n = Some(Arc::clone(&objects_copy[start + 1]));
                right_n = Some(Arc::clone(&objects_copy[start]));
            }
        } else {
            objects_copy.objects[start..end].sort_by(|a, b| comparator(a, b));
            // Integer division
            let mid = start + object_span / 2;
            left_n = Some(Arc::new(BVHNode::new(
                objects_copy,
                start,
                mid,
                time0,
                time1,
            )));
            right_n = Some(Arc::new(BVHNode::new(objects_copy, mid, end, time0, time1)));
        }

        let mut box_left: Aabb = Default::default();
        let mut box_right: Aabb = Default::default();

        if !left_n
            .as_ref()
            .unwrap()
            .bounding_box(0.0, 0.0, &mut box_left)
            || !right_n
                .as_ref()
                .unwrap()
                .bounding_box(0.0, 0.0, &mut box_right)
        {
            eprintln!("No bounding box in BVH_Node constructor!");
        }

        let final_box = box_left.surrounding_box(box_right);

        BVHNode {
            bbox: final_box,
            left: left_n,
            right: right_n,
        }
    }

    /*
    pub fn new(
        objects_copy: &mut HittableList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = {
            let mut ranges = [
                (0, BVHNode::axis_range(objects_copy, 0)),
                (1, BVHNode::axis_range(objects_copy, 1)),
                (2, BVHNode::axis_range(objects_copy, 2)),
            ];
            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        };
        objects_copy.objects[start..end].sort_unstable_by(|a, b| {
            let abb: Aabb = Default::default();
            let bbb: Aabb = Default::default();
            let av = abb.minimum[axis] + abb.maximum[axis];
            let bv = bbb.minimum[axis] + bbb.maximum[axis];
            av.partial_cmp(&bv).unwrap()
        });

        let left: Option<Arc<dyn Hittable>>;
        let right: Option<Arc<dyn Hittable>>;
        match objects_copy.objects.len() {
            0 => panic!("Can't create BVH for zero objects!"),
            1 => {
                let mut tmp_bbox: Aabb = Default::default();
                objects_copy[0].bounding_box(0.0, 0.0, &mut tmp_bbox);
                Self {
                    bbox: tmp_bbox,
                    left: Some(Arc::clone(&objects_copy[0])),
                    right: Some(Arc::clone(&objects_copy[0])),
                }
            }
            _ => {
                let mid = start + (end - start) / 2;
                right = Some(Arc::new(BVHNode::new(objects_copy, mid, end, 0.0, 0.0)));
                left = Some(Arc::new(BVHNode::new(objects_copy, start, mid, 0.0, 0.0)));
                let mut left_bbox: Aabb = Default::default();
                let mut right_bbox: Aabb = Default::default();
                left.as_ref()
                    .unwrap()
                    .bounding_box(time0, time1, &mut left_bbox);
                right
                    .as_ref()
                    .unwrap()
                    .bounding_box(time0, time1, &mut right_bbox);
                BVHNode {
                    bbox: left_bbox.surrounding_box(right_bbox),
                    left: left,
                    right: right,
                }
            }
        }
    }
    */
}

impl Hittable for BVHNode {
    /*
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min..t_max) {
            return None;
        }
        // Check left and right hit
        let left_hit = match &self.left {
            None => None,
            Some(node) => node.hit(r, t_min, t_max),
        };
        let final_hit = match &self.right {
            None => left_hit,
            Some(node) => {
                let mut furthest = t_max;
                if let Some(hit) = left_hit {
                    furthest = hit.t;
                }
                node.hit(r, t_min, furthest)
            }
        };

        final_hit
    }
    */
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min..t_max) {
            // Technically putting the same element twice to each leaf node...
            match (self.left.as_ref(), self.right.as_ref()) {
                (Some(node_l), Some(node_r)) => {
                    let hit_left = node_l.hit(r, t_min, t_max);

                    let mut temp_t_max = t_max;
                    if let Some(h) = &hit_left {
                        temp_t_max = h.t;
                    }

                    let hit_right = node_r.hit(r, t_min, temp_t_max);

                    match (hit_left, hit_right) {
                        (h, None) | (None, h) => h,
                        (Some(hl), Some(hr)) => {
                            if hl.t < hr.t {
                                Some(hl)
                            } else {
                                Some(hr)
                            }
                        }
                    }
                }
                _ => None,
            };
        }
        None
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

    if let Some(ord) = box_a.min()[axis].partial_cmp(&box_b.min()[axis]) {
        ord
    } else {
        Ordering::Greater
    }
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
