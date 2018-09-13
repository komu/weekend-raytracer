use ray::Ray;
use cgmath::{dot, Vector3};
use hitable::{Hitable, HitRecord};
use material::Material;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Rc<Material>
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
        }
        return false;
    }
}
