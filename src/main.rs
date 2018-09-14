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
use material::Material;
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
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let up = vec3(0.0, 1.0, 0.0);
    let aspect = nx as f64 / ny as f64;

    let camera = Camera::new(lookfrom, lookat, up, 20.0, aspect, aperture, dist_to_focus);
    let world = random_scene();

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
        col = col.map({ |v| { v.sqrt() } });

        let ir = (255.99 * col.x) as u8;
        let ig = (255.99 * col.y) as u8;
        let ib = (255.99 * col.z) as u8;

        image::Rgb([ir, ig, ib])
    });

    println!();
    img.save("images/output.png").unwrap();
}

fn random_scene() -> HitableList {
    let mut vec: Vec<Box<Hitable>> = vec![];

    vec.push(Box::new(Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))))));

    for a in -11..11 {
        for b in -11..11 {
            let center = vec3(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Rc<Material>;
                let choose_mat = random::<f64>();

                if choose_mat < 0.8 {
                    material = Rc::new(Lambertian::new(vec3(random::<f64>() * random::<f64>(), random::<f64>() * random::<f64>(), random::<f64>() * random::<f64>())));
                } else if choose_mat < 0.95 {
                    material = Rc::new(Metal::new(vec3(0.5 * (1.0 + random::<f64>()), 0.5 * (1.0 + random::<f64>()), 0.5 * (1.0 + random::<f64>())), 0.5 * random::<f64>()));
                } else {
                    material = Rc::new(Dielectric::new(1.5));
                }
                vec.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    vec.push(Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
    vec.push(Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))))));
    vec.push(Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)))));

    HitableList::new(vec)
}
