extern crate cgmath;
extern crate image;
extern crate num_cpus;
extern crate rand;

use bvh_node::BvhNode;
use camera::Camera;
use cgmath::{Point3, vec3};
use cgmath::prelude::*;
use color::Color;
use hitable::Hitable;
use hitable_list::HitableList;
use image::{ImageBuffer, Rgb};
use material::{Dielectric, Lambertian, Metal};
use rand::{random, Rng, thread_rng};
use ray::Ray;
use sphere::{MovingSphere, Sphere};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod aabb;
mod bvh_node;
mod camera;
mod color;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;

fn color<T: Hitable + ?Sized>(ray: &Ray, world: &T, depth: u32) -> Color {
    if let Some(rec) = world.hit(ray, 0.001, f64::max_value()) {
        if depth >= 50 {
            return Color::black();
        }

        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            let col = color(&scattered, world, depth + 1);
            return attenuation * col;
        } else {
            return Color::black();
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx = 600;
    let ny = 400;
    let ns = 20;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let up = vec3(0.0, 1.0, 0.0);
    let aspect = nx as f64 / ny as f64;

    let camera = Arc::new(Camera::new(lookfrom, lookat, up, 20.0, aspect, aperture, dist_to_focus, time0, time1));
    let mut rng = thread_rng();
    let world = Arc::new(random_scene(&mut rng, time0, time1));
    let now = Instant::now();

    let cpus = num_cpus::get();

    let arc_img = Arc::new(Mutex::new(ImageBuffer::new(nx, ny)));
    let mut threads = Vec::new();
    let y_counter = Arc::new(Mutex::new(0));

    for _ in 0..cpus {
        let arc_img = arc_img.clone();
        let camera = camera.clone();
        let y_counter = y_counter.clone();
        let world = world.clone();

        threads.push(thread::spawn(move || {
            let camera= &camera;
            let world = &*world;
            let mut row = Vec::with_capacity(nx as usize);

            loop {
                let y = get_and_increment(&y_counter);
                if y >= ny {
                    break;
                }

                print!("\r{}/{}", y + 1, ny);
                std::io::stdout().flush().expect("Could not flush stdout");

                row.clear();
                for x in 0..nx {
                    let i = x;
                    let j = ny - y;

                    let mut col = Color::black();
                    for _ in 0..ns {
                        let u = (i as f64 + random::<f64>()) / (nx as f64);
                        let v = (j as f64 + random::<f64>()) / (ny as f64);

                        let ray = camera.get_ray(u, v);
                        col += color(&ray, world, 0);
                    }

                    col /= ns as f64;
                    col = col.gamma_correct();

                    let ir = (255.99 * col.r) as u8;
                    let ig = (255.99 * col.g) as u8;
                    let ib = (255.99 * col.b) as u8;

                    row.push(Rgb([ir, ig, ib]));
                }

                let mut img = arc_img.lock().expect("could not lock image");
                for (x, color) in row.iter().enumerate() {
                    img.put_pixel(x as u32, y, *color);
                }
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let elapsed_seconds = now.elapsed().as_secs();
    let samples = (nx * ny * ns) as u64;
    println!("\nrendered {} samples in {} seconds ({} samples/s)", samples, elapsed_seconds, samples / elapsed_seconds);
    arc_img.lock().unwrap().save("images/output.png").unwrap();
}

fn get_and_increment(counter: &Arc<Mutex<u32>>) -> u32 {
    let mut shared_y = counter.lock().expect("locking counter failed");
    let value = *shared_y;
    *shared_y = value + 1;
    value
}

fn random_scene<T : Rng>(rng: &mut T, t0: f64, t1: f64) -> HitableList {
    let mut vec: Vec<Box<Hitable>> = vec![];

    vec.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))))));

    for a in -11..11 {
        for b in -11..11  {
            let center = Point3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let choose_mat = rng.gen::<f64>();

                if choose_mat < 0.8 {
                    vec.push(Box::new(MovingSphere::new(center, center + vec3(0.0, 0.5 * rng.gen::<f64>(), 0.0), 0.0, 1.0, 0.2, Arc::new(Lambertian::new(Color::new(rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>(), rng.gen::<f64>() * rng.gen::<f64>()))))));
                } else if choose_mat < 0.95 {
                    vec.push(Box::new(Sphere::new(center, 0.2, Arc::new(Metal::new(Color::new(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())), 0.5 * rng.gen::<f64>())))));
                } else {
                    vec.push(Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    vec.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    vec.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    vec.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)))));

    HitableList::new(vec![BvhNode::build(vec, t0, t1)])
}
