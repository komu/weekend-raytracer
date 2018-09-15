use aabb::AABB;
use cgmath::{Point3, Vector3};
use material::Material;
use ray::Ray;
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Arc<Material>,
}

pub trait Hitable : Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn bounding_box_required(&self, t0: f64, t1: f64) -> AABB {
        self.bounding_box(t0, t1).expect("no bounding box")
    }
}

impl HitRecord {
    pub fn new(t: f64, p: Point3<f64>, normal: Vector3<f64>, material: &Arc<Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material: material.clone()
        }
    }
}
