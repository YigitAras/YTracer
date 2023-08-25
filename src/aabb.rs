use crate::{ray::*, vector3::*};

#[derive(Default, Copy, Clone)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn min(self) -> Vec3 {
        self.minimum
    }

    pub fn surrounding_box(self, other: Aabb) -> Self {
        Self {
            minimum: self.minimum.zip_with(other.minimum, f64::min),
            maximum: self.maximum.zip_with(other.maximum, f64::max),
        }
    }
    #[allow(dead_code)]
    pub fn max(self) -> Vec3 {
        self.maximum
    }

    pub fn hit(self, r: Ray, t_range: std::ops::Range<f64>) -> bool {
        let inv_d = 1.0 / r.dir;
        let t0 = (self.minimum - r.orig) * inv_d;
        let t1 = (self.maximum - r.orig) * inv_d;
        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { a } else { b }),
        );
        let start = t_range.start.max(t0.reduce(f64::max));
        let end = t_range.end.min(t1.reduce(f64::min));
        end > start
    }
}
