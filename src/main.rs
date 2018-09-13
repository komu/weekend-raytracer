extern crate cgmath;
extern crate image;
extern crate rand;

use camera::Camera;
use cgmath::{vec3, Vector3};
use cgmath::prelude::*;
use hitable::Hitable;
use hitable_list::HitableList;
use image::ImageBuffer;
use material::Lambertian;
use material::Metal;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use std::io::prelude::*;
use std::rc::Rc;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;

fn main() {
    let nx = 600;
    let ny = 300;
    let ns = 100;

    let camera = Camera::new();
    let world = HitableList::new(vec!(
        Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(vec3(0.8, 0.3, 0.3))))),
        Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(vec3(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(vec3(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(vec3(0.8, 0.6, 0.2), 1.0)))),
        Box::new(Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(vec3(0.8, 0.8, 0.8), 0.3))))
    ));

    let mut previous_j = 0;
    let img = ImageBuffer::from_fn(nx, ny, |i, j| {
        if j != previous_j {
            previous_j = j;
            print!("\r{}/{}", j + 1, ny);
            std::io::stdout().flush().ok().expect("Could not flush stdout");
        }

        let j = ny - j;

        let mut col = vec3(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (i as f64 + random::<f64>()) / (nx as f64);
            let v = (j as f64 + random::<f64>()) / (ny as f64);

            let ray = camera.get_ray(u, v);
            col += color(&ray, &world, 0);
        }

        col /= ns as f64;
        col = col.map({ |v| { v.sqrt() }});

        let ir = (255.99 * col.x) as u8;
        let ig = (255.99 * col.y) as u8;
        let ib = (255.99 * col.z) as u8;

        image::Rgb([ir, ig, ib])
    });

    println!();
    img.save("images/foo.png").unwrap();
}

fn color<T: Hitable>(ray: &Ray, world: &T, depth: u32) -> Vector3<f64> {
    if let Some(rec) = world.hit(ray, 0.001, f64::max_value()) {
        if depth >= 50 {
            return vec3(0.0, 0.0, 0.0);
        }

        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            let col = color(&scattered, world, depth + 1);
            return vec3(
                attenuation.x * col.x,
                attenuation.y * col.y,
                attenuation.z * col.z);
        } else {
            return vec3(0.0, 0.0, 0.0);
        }

    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
    }
}
