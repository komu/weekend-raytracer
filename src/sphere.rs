use aabb::AABB;
use cgmath::Point3;
use cgmath::prelude::*;
use cgmath::vec3;
use hitable::{Hitable, HitRecord};
use material::Material;
use ray::Ray;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let pt = ray.point_at(t);
                return Some(HitRecord::new(t, pt, (pt - self.center) / self.radius, &self.material));
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let pt = ray.point_at(t);
                return Some(HitRecord::new(t, pt, (pt - self.center) / self.radius, &self.material));
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(sphere_box(&self.center, self.radius))
    }
}

fn sphere_box(center: &Point3<f64>, radius: f64) -> AABB {
    let v = vec3(radius, radius, radius);
    AABB::new(center - v, center + v)
}

pub struct MovingSphere {
    pub center0: Point3<f64>,
    pub center1: Point3<f64>,
    time0: f64,
    time1: f64,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl MovingSphere {
    pub fn new(center0: Point3<f64>, center1: Point3<f64>, time0: f64, time1: f64, radius: f64, material: Arc<Material>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point3<f64> {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let pt = ray.point_at(t);
                return Some(HitRecord::new(t, pt, (pt - center) / self.radius, &self.material));
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let pt = ray.point_at(t);
                return Some(HitRecord::new(t, pt, (pt - center) / self.radius, &self.material));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box1 = sphere_box(&self.center(t0), self.radius);
        let box2 = sphere_box(&self.center(t1), self.radius);
        Some(box1.union(&box2))
    }
}
