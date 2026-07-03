use std::f64::consts::PI;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vector;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
