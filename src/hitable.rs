use cgmath::Vector3;
use material::Material;
use ray::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Rc<Material>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(t: f64, p: Vector3<f64>, normal: Vector3<f64>, material: &Rc<Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material: material.clone()
        }
    }
}
