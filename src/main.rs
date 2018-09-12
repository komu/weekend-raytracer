use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let file = File::create("images/bar.ppm")?;
    let ref mut ppm = BufWriter::new(file);

    ppm.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2;
            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;

            ppm.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }

    Ok(())
}
