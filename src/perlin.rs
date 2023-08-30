/*
 *   Struct and necessary methods to create Perlin noise
 */
use crate::vector3::*;

use crate::utils::random_vec;
use rand::Rng;

pub struct Perlin {
    ran_vec: Vec<Vec3>,
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
        let _rng = rand::thread_rng();

        let mut ran_vec: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); Perlin::point_count()];

        for rvec in ran_vec.iter_mut().take(Perlin::point_count()) {
            *rvec = Vec3::unit_vector(random_vec(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            ran_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - f64::floor(p.x);
        let v = p.y - f64::floor(p.y);
        let w = p.z - f64::floor(p.z);

        let i = f64::floor(p.x) as i64;
        let j = f64::floor(p.y) as i64;
        let k = f64::floor(p.z) as i64;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2i64 {
            for dj in 0..2i64 {
                for dk in 0..2i64 {
                    c[di as usize][dj as usize][dk as usize] = self.ran_vec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize]
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }

    // TODO: Rustify the loop?
    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2i64 {
            for j in 0..2i64 {
                for k in 0..2i64 {
                    let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));
                    accum += (i as f64 * uu + ((1 - i) as f64) * (1.0 - uu))
                        * (j as f64 * vv + ((1 - j) as f64) * (1.0 - vv))
                        * (k as f64 * ww + ((1 - k) as f64) * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize].dot(weight_v);
                }
            }
        }
        accum
    }

    // TODO: Rustify the loop? This one would be difficult actually
    fn permute(p: &mut [i64], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let target: usize = rng.gen_range(0..i);
            // Todo: Not sure of this operation
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm() -> Vec<i64> {
        let mut p: Vec<i64> = vec![0; Perlin::point_count()];

        for (i, point) in p.iter_mut().enumerate().take(Perlin::point_count()) {
            *point = i as i64;
        }

        Perlin::permute(&mut p, Perlin::point_count());

        p
    }
}
