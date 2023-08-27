/*
 *   Struct and necessary methods to create Perlin noise
 */
use crate::vector3::*;

use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Perlin {
    rand_floats: Vec<f64>,
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>
}

impl Perlin {
    // To return the number of points
    #[inline]
    pub fn point_count() -> usize {
        256
    }

    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn noise(&self, p: Vec3) -> f64{
        let i = ((4.0 * p.x) as i64) & 255;
        let j = ((4.0 * p.y) as i64) & 255;
        let k = ((4.0 * p.z) as i64) & 255;

        self.rand_floats[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    // TODO: Rustify the loop?
    fn permute(p: &mut [i64], n: usize) {
        let mut rng = rand::thread_rng();
        for i in  (1..n).rev() {
            let target: usize = rng.gen_range(0..i);
            // Todo: Not sure of this operation
            p.swap(i, target);
        }
    }
    // TODO: Rustify the loop?
    fn perlin_generate_perm() -> Vec<i64> {
        let mut p: Vec<i64> = vec![0; Perlin::point_count()];
        for i in 0..Perlin::point_count() {
            p[i] = i as i64;
        }

        Perlin::permute(&mut p, Perlin::point_count());

        p
    }
}