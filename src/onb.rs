// Orthonormal Basis Struct
use crate::{vector3::*};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Onb {
    pub fn u(self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    pub fn build_from_w(w: Vec3) -> Self {
        let unit_w = Vec3::unit_vector(w);
        let a = if f64::abs(unit_w.x) > 0.9 {
            Vec3::new(0.0 ,1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::unit_vector(unit_w.cross(a));
        let u = unit_w.cross(v);

        Self {
            axis: [u, v, unit_w]
        }

    }
}

impl Index<usize> for Onb {
    type Output = Vec3;
    #[inline]
    fn index(&self, i: usize) -> &Vec3 {
        match i {
            0 => &self.axis[0],
            1 => &self.axis[1],
            2 => &self.axis[2],
            _ => &self.axis[2], // Well should probably throw an error
        }
    }
}
impl IndexMut<usize> for Onb {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Vec3 {
        match i {
            0 => &mut self.axis[0],
            1 => &mut self.axis[1],
            2 => &mut self.axis[2],
            _ => &mut self.axis[2], // Well should probably throw an error
        }
    }
}
