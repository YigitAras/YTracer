use std::sync::Arc;

use crate::vector3::*;
pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}
#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn new(c: Vec3) -> Self {
        Self { color_value: c }
    }
    pub fn from_color(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

// Simply return the solid color
impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Vec3 {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture{
    pub fn from_tex(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self{
        Self {
            odd,
            even
        }
    }
    pub fn from_color(c1: Vec3, c2: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2))
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        todo!()
    }
}
