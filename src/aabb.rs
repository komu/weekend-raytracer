use cgmath::{Point3, Vector3};
use cgmath::prelude::*;
use ray::Ray;

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl AABB {
    pub fn new(min: Point3<f64>, max: Point3<f64>) -> AABB {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        let inv_dir = Vector3::new(1.0, 1.0, 1.0).div_element_wise(ray.direction);

        for a in 0..3 {
            let t1 = (self.min[a] - ray.origin[a]) * inv_dir[a];
            let t2 = (self.max[a] - ray.origin[a]) * inv_dir[a];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax > tmin
    }

    pub fn union(&self, box1: &AABB) -> AABB {
        AABB {
            min: Point3::new(self.min.x.min(box1.min.x),
                             self.min.y.min(box1.min.y),
                             self.min.z.min(box1.min.z)),
            max: Point3::new(self.max.x.max(box1.max.x),
                             self.max.y.max(box1.max.y),
                             self.max.z.max(box1.max.z))
        }
    }
}
