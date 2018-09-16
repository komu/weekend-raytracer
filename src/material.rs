use cgmath::{dot, vec3, Vector3};
use cgmath::prelude::*;
use color::Color;
use hitable::HitRecord;
use rand::random;
use ray::Ray;
use texture::Texture;

pub trait Material : Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Box<Texture>
}

impl Lambertian {
    pub fn new(albedo: Box<Texture>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p, r_in.time);

        Some((scattered, self.albedo.value(0.0, 0.0, &rec.p)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), r_in.time);

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::white();
        let outward_normal: Vector3<f64>;
        let ni_over_nt: f64;
        let cosine: f64;

        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * r_in.direction.dot(rec.normal) / r_in.direction.magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -dot(r_in.direction, rec.normal) / r_in.direction.magnitude();
        }


        if let Some(refracted) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refraction_index);

            if random::<f64>() >= reflect_prob {
                return Some((Ray::new(rec.p, refracted, r_in.time), attenuation))
            }
        }

        let reflected = reflect(&r_in.direction, &rec.normal);
        Some((Ray::new(rec.p, reflected, r_in.time), attenuation))
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

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(*n) * n
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
