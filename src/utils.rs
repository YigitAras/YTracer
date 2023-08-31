use crate::vector3::*;
use rand::Rng;
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.lenght_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_vec(l: f64, h: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3 {
        x: rng.gen_range(l..h),
        y: rng.gen_range(l..h),
        z: rng.gen_range(l..h),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - unit;
        if p.lenght_squared() < 1.0 {
            return p;
        }
    }
}
