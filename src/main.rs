extern crate cgmath;
extern crate image;

mod hitable;
mod hitable_list;
mod ray;
mod sphere;

use cgmath::prelude::*;
use cgmath::{vec3, Vector3};
use ray::Ray;
use image::ImageBuffer;
use hitable::{Hitable, HitRecord};
use hitable_list::HitableList;
use sphere::Sphere;

fn main() {
    let nx = 600;
    let ny = 300;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);
    let world = HitableList::new(vec!(
        Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0))
    ));

    let img = ImageBuffer::from_fn(nx, ny, |i, j| {
        let j = ny - j;
        let u = (i as f64) / (nx as f64);
        let v = (j as f64) / (ny as f64);

        let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let c = color(&r, &world);
        let ir = (255.99 * c.x) as u8;
        let ig = (255.99 * c.y) as u8;
        let ib = (255.99 * c.z) as u8;

        image::Rgb([ir, ig, ib])
    });

    img.save("images/foo.png").unwrap();
}

fn color<T: Hitable>(ray: &Ray, world: &T) -> Vector3<f64> {
    let mut rec = HitRecord::new(f64::max_value());

    if world.hit(ray, 0.0, f64::max_value(), &mut rec) {
        let n = rec.normal;
        return 0.5 * vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
    }
}
