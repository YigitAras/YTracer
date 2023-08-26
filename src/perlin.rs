/*
 *   Struct and necessary methods to create Perlin noise
 */
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

    pub fn noise(&self) -> f64{
        0.0
    }

    fn permute(p: &Vec<i64>, n: usize) {
        unimplemented!()
    }
    fn perlin_generate_perm() -> Vec<i64> {
        let mut p: Vec<i64> = vec![0; Perlin::point_count()];
        for i in 0..Perlin::point_count() {
            p[i] = i as i64;
        }

        Perlin::permute(&p, Perlin::point_count());
        
        p
    }
}