use cgmath::{dot, vec3, Vector3};
use cgmath::prelude::*;
use hitable::HitRecord;
use rand::random;
use ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Vector3<f64>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);

        return Some((scattered, self.albedo));
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let v = 2.0 * vec3(random::<f64>(), random::<f64>(), random::<f64>()) - vec3(1.0, 1.0, 1.0);
        if v.magnitude2() >= 1.0 {
            return v;
        }
    }
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * dot(v, n) * n
}
