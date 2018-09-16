use array_init::array_init;
use cgmath::{Point3, prelude::*, vec3, Vector3};
use color::Color;
use rand::{random, Rng, thread_rng};
use texture::Texture;

pub struct Perlin {
    vectors: [Vector3<f64>; 256],
    perm_x: Permutation,
    perm_y: Permutation,
    perm_z: Permutation,
}

impl Perlin {
    fn new() -> Perlin {
        Perlin {
            vectors: array_init(|_| { vec3(random_around_zero(), random_around_zero(), random_around_zero()).normalize() }),
            perm_x: Permutation::new(),
            perm_y: Permutation::new(),
            perm_z: Permutation::new(),
        }
    }

    fn noise(&self, p: Vector3<f64>) -> f64 {
        let u = fraction(p.x);
        let v = fraction(p.y);
        let w = fraction(p.z);

        let (x0, x1) = self.perm_x.lookup(p.x);
        let (y0, y1) = self.perm_y.lookup(p.y);
        let (z0, z1) = self.perm_z.lookup(p.z);

        let c = [
            [[self.vectors[x0 ^ y0 ^ z0], self.vectors[x0 ^ y0 ^ z1]], [self.vectors[x0 ^ y1 ^ z0], self.vectors[x0 ^ y1 ^ z1]]],
            [[self.vectors[x1 ^ y0 ^ z0], self.vectors[x1 ^ y0 ^ z1]], [self.vectors[x1 ^ y1 ^ z0], self.vectors[x1 ^ y1 ^ z1]]]
        ];

        perlin_interp(&c, u, v, w)
    }

    fn turb(&self, p: Vector3<f64>) -> f64 {
        self.turb_depth(p, 7)
    }

    fn turb_depth(&self, mut p: Vector3<f64>, depth: u32) -> f64 {
        let mut accum: f64 = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }
}

struct Permutation {
    data: [usize; 256]
}

impl Permutation {
    fn new() -> Permutation {
        let mut data: [usize; 256] = array_init(|i| { i });
        thread_rng().shuffle(&mut data);
        Permutation { data }
    }

    #[inline]
    fn lookup(&self, index: f64) -> (usize, usize) {
        let i = index.floor() as usize;

        (self.data[i % 256], self.data[(i + 1) % 256])
    }
}

fn random_around_zero() -> f64 {
    -1.0 + 2.0 * random::<f64>()
}

pub struct NoiseTexture {
    scale: f64,
    perlin: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            scale,
            perlin: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3<f64>) -> Color {
//        let s = 0.5 * (1.0 + self.perlin.noise(self.scale * p.to_vec()));
//        let s = self.perlin.turb(self.scale * p.to_vec());
        let s = 0.5 * (self.scale * p.z + 10.0 * self.perlin.turb(p.to_vec())).sin();
        s * Color::white()
    }
}

fn perlin_interp(c: &[[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = hermite_cubic(u);
    let vv = hermite_cubic(v);
    let ww = hermite_cubic(w);
    let mut sum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ii = i as f64;
                let jj = j as f64;
                let kk = k as f64;
                let weight_v = vec3(u - ii, v - jj, w - kk);
                sum += (ii * uu + (1.0 - ii) * (1.0 - uu))
                    * (jj * vv + (1.0 - jj) * (1.0 - vv))
                    * (kk * ww + (1.0 - kk) * (1.0 - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    sum
}

#[inline]
fn hermite_cubic(x: f64) -> f64 {
    x * x * (3.0 - 2.0 * x)
}

#[inline]
fn fraction(x: f64) -> f64 {
    x - x.floor()
}
