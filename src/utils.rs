use rand::rngs::ThreadRng;
use rand::Rng;
use crate::vector3::*;

const PI: f64  = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {
        return min;
    }
    if x>max {
        return max;
    }
    x
}

pub fn random_vec(min: f64, max: f64, rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(
     rng.gen_range(min..max),
     rng.gen_range(min..max),
     rng.gen_range(min..max)
    )
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = random_vec(-1.0, 1.0, rng);
        if p.lenght_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    return Vec3::unit_vector(random_in_unit_sphere(rng));
}