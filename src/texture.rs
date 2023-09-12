use std::sync::Arc;

use crate::geometry::vector3::*;
use crate::perlin::*;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}
#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn from_color(c: Vec3) -> Self {
        Self { color_value: c }
    }
    #[allow(dead_code)]
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
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

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn from_tex(even: Arc<dyn Texture>, odd: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { odd, even }
    }
    pub fn from_color(c1: Vec3, c2: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::from_color(c1)),
            even: Arc::new(SolidColor::from_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        // let sines = sin(10*p.x())*sin(10*p.y())*sin(10*p.z());
        let sines = f64::sin(10.0 * point.x) * f64::sin(10.0 * point.y) * f64::sin(10.0 * point.z);
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(point.z * self.scale + 10.0 * self.noise.turbulance(point, 7)))
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u64,
    height: u64,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: u64, height: u64) -> Self {
        ImageTexture {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Vec3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        let nx = self.width as usize;
        let ny = self.height as usize;
        let mut i = (u * nx as f64) as usize;
        let mut j = ((1.0 - v) * ny as f64) as usize;
        if i > nx - 1 {
            i = nx - 1
        }
        if j > ny - 1 {
            j = ny - 1
        }
        let idx = 3 * i + 3 * nx * j;
        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;
        Vec3::new(r, g, b)
    }
}
