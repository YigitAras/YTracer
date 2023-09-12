use std::sync::Arc;

use crate::accelerators::aabb::*;
use crate::core::hittable::*;
use crate::core::hittable_list::*;
use crate::geometry::ray::*;

#[allow(dead_code)]
enum BVHNode {
    Branch { left: Arc<Bvh>, right: Arc<Bvh> },
    Leaf(Arc<dyn Hittable>),
}

pub struct Bvh {
    pub bbox: Aabb,
    tree: BVHNode,
}
// TODO: Make sure the ranges time0..time1 are forwarded properly when implemented
impl Bvh {
    #[allow(dead_code)]
    #[inline]
    fn axis_selection(objs: &[Arc<dyn Hittable>]) -> usize {
        fn axis_range(objs: &[Arc<dyn Hittable>], axis: usize) -> f64 {
            let range = objs.iter().fold(f64::MAX..f64::MIN, |range, o| {
                let mut bb: Aabb = Default::default();
                o.bounding_box(0.0, 0.0, &mut bb);
                let min = bb.minimum[axis].min(bb.maximum[axis]);
                let max = bb.minimum[axis].max(bb.maximum[axis]);
                range.start.min(min)..range.end.max(max)
            });
            range.end - range.start
        }
        {
            let mut ranges = [
                (0, axis_range(objs, 0)),
                (1, axis_range(objs, 0)),
                (2, axis_range(objs, 0)),
            ];
            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        }
    }
    #[allow(dead_code)]
    pub fn new(
        objects_copy: &mut HittableList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let object_span = end - start;
        // Make sure only the slice of data that is being used is looked up for axis
        let axis = Bvh::axis_selection(&objects_copy.objects[start..end]);

        // Better axis selection for separation
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            let mut abb: Aabb = Default::default();
            let mut bbb: Aabb = Default::default();
            a.bounding_box(0.0, 0.0, &mut abb);
            b.bounding_box(0.0, 0.0, &mut bbb);
            let av = abb.minimum[axis] + abb.maximum[axis];
            let bv = bbb.minimum[axis] + bbb.maximum[axis];
            av.partial_cmp(&bv).unwrap()
        };

        // Sort the objects
        objects_copy.objects[start..end].sort_unstable_by(comparator);

        let size = objects_copy.objects[start..end].len();

        match size {
            0 => panic!["No elements in scene!"],
            // Let each leaf have 10 triangles
            1..=10 => {
                let mut items: Vec<Arc<dyn Hittable>> = vec![];
                objects_copy.objects[start..end].clone_into(&mut items);
                let mut tmp_bbox: Aabb = Default::default();
                let leaf: Arc<dyn Hittable> = Arc::new(HittableList::from(items));
                if leaf.bounding_box(time0, time1, &mut tmp_bbox) {
                    Bvh {
                        tree: BVHNode::Leaf(leaf),
                        bbox: tmp_bbox,
                    }
                } else {
                    panic!["No bounding box in BVH Node"]
                }
            }
            _ => {
                let mid = start + object_span / 2;
                let left = Bvh::new(objects_copy, start, mid, time0, time1);
                let right = Bvh::new(objects_copy, mid, end, time0, time1);
                let tmp_bbox = left.bbox.surrounding_box(right.bbox);
                Bvh {
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

impl Hittable for Bvh {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min..t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
                BVHNode::Branch { left, right } => {
                    let mut temp_t_max = t_max;
                    let left = left.hit(r, t_min, t_max);
                    if let Some(l) = &left {
                        temp_t_max = l.t
                    };
                    let right = right.hit(r, t_min, temp_t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
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
