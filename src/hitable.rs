use ray::Ray;
use cgmath::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new(t: f64) -> HitRecord {
        HitRecord {
            t,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}
