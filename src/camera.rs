use cgmath::{Point3, vec3, Vector3};
use cgmath::prelude::*;
use rand::random;
use ray::Ray;
use std::f64::consts::PI;

pub struct Camera {
    lower_left: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    origin: Point3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3<f64>,
               lookat: Vector3<f64>,
               vup: Vector3<f64>,
               vfov: f64,
               aspect: f64,
               aperture: f64,
               focus_dist: f64,
               time0: f64,
               time1: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom.to_vec() - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            lower_left: lookfrom.to_vec() - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u,
            v,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + random::<f64>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin.to_vec() - offset,
            time,
        )
    }
}

fn random_in_unit_disc() -> Vector3<f64> {
    loop {
        let v = 2.0 * vec3(random::<f64>(), random::<f64>(), random::<f64>()) - vec3(1.0, 1.0, 0.0);
        if v.dot(v) >= 1.0 {
            return v;
        }
    }
}
