use std::{io::BufWriter, io::Write};

use crate::vector3::*;

pub fn write_color(buff: &mut BufWriter<std::fs::File>, color: Vec3) {
    let ir = (255.999 * color.x) as u64;
    let ig = (255.999 * color.y) as u64;
    let ib = (255.999 * color.z) as u64;

    let tmp_data: String = format!("{ir} {ig} {ib}\n");
    buff.write(tmp_data.as_bytes())
        .expect("Failed to write line of pixel data...");
}
