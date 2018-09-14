extern crate cgmath;
extern crate image;
extern crate rand;

use camera::Camera;
use cgmath::{vec3, Vector3};
use cgmath::prelude::*;
use hitable::Hitable;
use hitable_list::HitableList;
use image::ImageBuffer;
use material::{Dielectric, Lambertian, Metal};
use rand::{random, Rng, thread_rng};
use ray::Ray;
use sphere::{MovingSphere, Sphere};
use std::io::prelude::*;
use std::rc::Rc;
use std::time::Instant;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;

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

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 10;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let up = vec3(0.0, 1.0, 0.0);
    let aspect = nx as f64 / ny as f64;

    let camera = Camera::new(lookfrom, lookat, up, 20.0, aspect, aperture, dist_to_focus, 0.0, 1.0);
    let mut rng = thread_rng();
    let world = random_scene(&mut rng);
    let now = Instant::now();

    let mut previous_j = 0;
    let img = ImageBuffer::from_fn(nx, ny, |i, j| {
        if j != previous_j {
            previous_j = j;
            print!("\r{}/{}", j + 1, ny);
            std::io::stdout().flush().expect("Could not flush stdout");
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
        col = col.map({ |v| { v.sqrt() } });

        let ir = (255.99 * col.x) as u8;
        let ig = (255.99 * col.y) as u8;
        let ib = (255.99 * col.z) as u8;

        image::Rgb([ir, ig, ib])
    });

    let elapsed_seconds = now.elapsed().as_secs();
    let samples = (nx * ny * ns) as u64;
    println!("\nrendered {} samples in {} seconds ({} samples/s)", samples, elapsed_seconds, samples / elapsed_seconds);
    img.save("images/output.png").unwrap();
}

fn random_scene<T : Rng>(rng: &mut T) -> HitableList {
    let mut vec: Vec<Box<Hitable>> = vec![];

    vec.push(Box::new(Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))))));

    for a in -11..11 {
        for b in -11..11  {
            let center = vec3(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let choose_mat = rng.gen::<f64>();

                if choose_mat < 0.8 {
                    vec.push(Box::new(MovingSphere::new(center, center + vec3(0.0, 0.5 * rng.gen::<f64>(), 0.0), 0.0, 1.0, 0.2, Rc::new(Lambertian::new(vec3(rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>()))))));
                } else if choose_mat < 0.95 {
                    vec.push(Box::new(Sphere::new(center, 0.2, Rc::new(Metal::new(vec3(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())), 0.5 * rng.gen::<f64>())))));
                } else {
                    vec.push(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    vec.push(Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
    vec.push(Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))))));
    vec.push(Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)))));

    HitableList::new(vec)
}
