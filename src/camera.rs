use cgmath::prelude::*;
use cgmath::Vector3;
use ray::Ray;
use std::f64::consts::PI;

pub struct Camera {
    pub lower_left: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub origin: Vector3<f64>,
}

impl Camera {
    pub fn new(lookfrom: Vector3<f64>, lookat: Vector3<f64>, vup: Vector3<f64>, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            lower_left: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width *  u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
