extern crate cgmath;

mod ray;

use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use cgmath::prelude::*;
use cgmath::{dot, vec3, Vector3};
use ray::Ray;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    let file = File::create("images/bar.ppm")?;
    let ref mut ppm = BufWriter::new(file);

    ppm.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let c = color(&r);
            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;

            ppm.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }

    Ok(())
}

fn color(r: &Ray) -> Vector3<f64> {
    if hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, r) {
        return vec3(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

fn hit_sphere(ref center: Vector3<f64>, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(oc, r.direction);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}