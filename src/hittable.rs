use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point, Vector3};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point, t: f64, mat: Rc<dyn Material>, front_face: bool) -> Self {
        Self {
            p,
            normal: Vector3::new(0.0, 0.0, 0.0),
            mat,
            t,
            front_face,
        }
    }

    /// Sets the hit record normal vector
    /// NOTE: parameter `outward_normal` is assumed to have unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
