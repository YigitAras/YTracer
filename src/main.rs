use std::{fs::OpenOptions, io::BufWriter, io::Write};

fn main() {
    println!("Program started...\n");
    // Image related
    const IMAGE_WIDTH: u32 = 1028;
    const IMAGE_HEIGHT: u32 = 1028;

    // File
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("./outputs/IMAGE.ppm")
        .expect("Unable to open the file IMAGE.ppm");

    // Buffered writer for speed
    // Can keep everything in memory too
    let mut file = BufWriter::new(file);
    let data = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    file.write_all(data.as_bytes())
        .expect("Unable to write the header!");

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        println!("\rScanlines remaining: {j}");
        std::io::stdout()
            .flush()
            .expect("Flushing the stdout failed...\n");
        for i in 0..IMAGE_WIDTH {
            let r: f64 = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g: f64 = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b: f64 = 0.25;

            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;

            let tmp_data: String = format!("{ir} {ig} {ib}\n");
            file.write(tmp_data.as_bytes())
                .expect("Failed to write line of pixel data...");
        }
    }
    println!("Program ended...\n");
}
