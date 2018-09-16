use cgmath::Point3;
use color::Color;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3<f64>) -> Color;
}

impl Texture for Color {
    fn value(&self, _u: f64, _v: f64, _p: &Point3<f64>) -> Color {
        *self
    }
}

pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3<f64>) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
