use crate::{ray::*, vector3::*};

#[derive(Default, Copy, Clone)]
struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn min(self) -> Vec3 {
        self.minimum
    }

    pub fn max(self) -> Vec3 {
        self.maximum
    }
    pub fn hit(self, r: Ray, t_min: &mut f64, t_max: &mut f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self.min()[a] - r.orig[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.orig[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            *t_min = if t0 > *t_min { t0 } else { *t_min };
            *t_max = if t1 < *t_max { t1 } else { *t_max };

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
