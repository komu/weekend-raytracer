use aabb::AABB;
use hitable::{Hitable, HitRecord};
use ray::Ray;

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for element in &self.list {
            if let Some(rec) = element.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.list.is_empty() { return None; }

        if let Some(first) = self.list[0].bounding_box(t0, t1) {
            let mut result = first;

            for item in &self.list[1..] {
                if let Some(bb) = item.bounding_box(t0, t1) {
                    result = result.union(&bb);
                } else {
                    return None;
                }
            }

            Some(result)
        } else {
            None
        }
    }
}
