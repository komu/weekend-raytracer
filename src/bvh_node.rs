use aabb::AABB;
use hitable::Hitable;
use hitable::HitRecord;
use rand::{Rng, thread_rng};
use ray::Ray;

pub struct BvhNode {
    aabb: AABB,
    left: Box<Hitable>,
    right: Box<Hitable>,
}

impl BvhNode {
    fn new(aabb: AABB, left: Box<Hitable>, right: Box<Hitable>) -> BvhNode {
        BvhNode { aabb, left, right }
    }

    pub fn build(mut hitables: Vec<Box<Hitable>>, t0: f64, t1: f64) -> Box<Hitable> {
        let axis_index = thread_rng().gen_range(0, 3);
        hitables.sort_by(|a, b| {
            let lhs = &a.bounding_box_required(0.0, 0.0).min;
            let rhs = &b.bounding_box_required(0.0, 0.0).min;

            lhs[axis_index].partial_cmp(&rhs[axis_index]).unwrap()
        });

        let n = hitables.len();
        match n {
            0 => panic!("empty list"),
            1 => {
                let hitable = hitables.remove(0);
                Box::new(CachedBoundingBox::new(hitable.bounding_box_required(t0, t1), hitable))
            }
            2 => {
                let right = hitables.remove(1);
                let left = hitables.remove(0);
                let bb = left.bounding_box_required(t0, t1).union(&right.bounding_box_required(t0, t1));
                Box::new(BvhNode::new(bb, left, right))
            }
            _ => {
                let head = hitables.drain(0..n / 2).collect();
                let left = BvhNode::build(head, t0, t1);
                let right = BvhNode::build(hitables, t0, t1);
                let bb = left.bounding_box_required(t0, t1).union(&right.bounding_box_required(t0, t1));
                Box::new(BvhNode::new(bb, left, right))
            }
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = self.right.hit(ray, t_min, t_max);

            match (hit_left, hit_right) {
                (Some(l), Some(r)) => Some(if l.t < r.t { l } else { r }),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}

struct CachedBoundingBox {
    aabb: AABB,
    hitable: Box<Hitable>,
}

impl CachedBoundingBox {
    fn new(aabb: AABB, hitable: Box<Hitable>) -> CachedBoundingBox {
        CachedBoundingBox { aabb, hitable }
    }
}

impl Hitable for CachedBoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            self.hitable.hit(ray, t_min, t_max)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
