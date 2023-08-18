use crate::vector3::*;
pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3
}

impl SolidColor {
    pub fn new(c: Vec3) -> Self {
        Self {
            color_value: c
        }
    }
    pub fn from_color(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue)
        }
    }
}

// Simply return the solid color
impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        self.color_value
    }
}