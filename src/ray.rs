extern crate cgmath;

use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Ray {
        Ray {
            origin, direction, time
        }
    }

    pub fn point_at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}
