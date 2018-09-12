extern crate cgmath;

use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use cgmath::vec3;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let file = File::create("images/bar.ppm")?;
    let ref mut ppm = BufWriter::new(file);

    ppm.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = 255.99 * vec3((i as f64) / (nx as f64), (j as f64) / (ny as f64), 0.2);
            let ir = v.x as i32;
            let ig = v.y as i32;
            let ib = v.z as i32;

            ppm.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }

    Ok(())
}
