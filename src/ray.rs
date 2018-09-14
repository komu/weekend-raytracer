extern crate cgmath;

use cgmath::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn point_at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}
