
use crate::geometry::ray::*;
use crate::geometry::vector3::*;

#[derive(Default, Copy, Clone)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn surrounding_box(self, other: Aabb) -> Self {
        Self {
            minimum: self.minimum.zip_with(other.minimum, f64::min),
            maximum: self.maximum.zip_with(other.maximum, f64::max),
        }
    }
    #[inline]
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

    pub fn pad(&self) -> Self {
        let x_diff = self.maximum[0] - self.minimum[0];
        let y_diff = self.maximum[1] - self.minimum[1];
        let z_diff = self.maximum[2] - self.minimum[2];


        let delta = 0.0001;
        let padding = delta/2.0;

        let mut new_min = self.minimum;
        let mut new_max = self.maximum;

        // Pad the dimensions that are close to 0
        if x_diff <= delta {
            new_min[0] -= padding;
            new_max[0] += padding;
        }
        if y_diff <= delta {
            new_min[1] -= padding;
            new_max[1] += padding;
        }
        if z_diff <= delta {
            new_min[2] -= padding;
            new_max[2] += padding;
        }

        Self {
            minimum: new_min,
            maximum: new_max
        }
    }
}
