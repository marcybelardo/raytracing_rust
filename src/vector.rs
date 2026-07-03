use rand::prelude::*;
use std::{fmt::Display, ops};

pub type Point = Vector3;
pub type Color = Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, v: &Vector3) -> f64 {
        self.x * v.x() + self.y * v.y() + self.z * v.z()
    }

    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z() - self.z * v.y(),
            y: self.z * v.x() - self.x * v.z(),
            z: self.x * v.y() - self.y * v.x(),
        }
    }

    pub fn normalize(&self) -> Vector3 {
        *self / self.length()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}

pub fn random_vec() -> Vector3 {
    let mut rng = rand::rng();
    Vector3::new(rng.random(), rng.random(), rng.random())
}

pub fn random_vec_range(min: f64, max: f64) -> Vector3 {
    let mut rng = rand::rng();
    Vector3::new(
        rng.random_range(min..max),
        rng.random_range(min..max),
        rng.random_range(min..max),
    )
}

pub fn random_unit_vec() -> Vector3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            break p / f64::sqrt(lensq);
        }
    }
}

pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
    let on_unit_sphere = random_unit_vec();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = (-*uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;

    r_out_perp + r_out_parallel
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= rhs.recip()
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x(),
            y: self.y * rhs.y(),
            z: self.z * rhs.z(),
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x(),
            y: self * rhs.y(),
            z: self * rhs.z(),
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        rhs.recip() * self
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
