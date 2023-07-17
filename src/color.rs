use std::{io::BufWriter, io::Write};

use crate::vector3::*;
use crate::utils::*;

pub fn write_color(buff: &mut BufWriter<std::fs::File>, color: Vec3, samples_per_pixel: u64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    // Divide by the number of samples
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u64;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u64;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u64;



    let tmp_data: String = format!("{ir} {ig} {ib}\n");
    buff.write(tmp_data.as_bytes())
        .expect("Failed to write line of pixel data...");
}
