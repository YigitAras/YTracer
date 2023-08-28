/*
 *   Struct and necessary methods to create Perlin noise
 */
use crate::vector3::*;

use rand::Rng;

pub struct Perlin {
    rand_floats: Vec<f64>,
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>,
}

impl Perlin {
    // To return the number of points
    #[inline]
    pub fn point_count() -> usize {
        256
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut rand_floats = vec![0.0; Perlin::point_count()];
        for i in 0..Perlin::point_count() {
            rand_floats[i] = rng.gen_range(0.0..1.0);
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            rand_floats,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let mut u = p.x - f64::floor(p.x);
        let mut v = p.y - f64::floor(p.y);
        let mut w = p.z - f64::floor(p.z);
        // Hermitian Smoothing?
        u = u * u*(3.0-2.0*u);
        v = v * v*(3.0-2.0*v);
        w = w * w*(3.0-2.0*w);


        let i = f64::floor(p.x) as i64;
        let j = f64::floor(p.y) as i64;
        let k = f64::floor(p.z) as i64;
        let mut c = [[[0.0f64; 2]; 2]; 2];
        for di in 0..2i64 {
            for dj in 0..2i64 {
                for dk in 0..2i64 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_floats[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize]
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2i64 {
            for j in 0..2i64 {
                for k in 0..2i64 {
                    accum += (i as f64 * u + ((1 - i) as f64) * (1.0 - u))
                        * (j as f64 * v + ((1 - j) as f64) * (1.0 - v))
                        * (k as f64 * w + ((1 - k) as f64) * (1.0 - w))
                        * c[i as usize][j as usize][k as usize];
                }
            }
        }
        accum
    }
    // TODO: Rustify the loop?
    fn permute(p: &mut [i64], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
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
