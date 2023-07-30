use crate::{ray::*, vector3::*};

struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn min(self) -> Vec3 {
        self.minimum
    }

    pub fn max(self) -> Vec3 {
        self.maximum
    }
    pub fn hit(_r: Ray, _t_min: f64, _t_max: f64) -> bool {
        false
    }
}
