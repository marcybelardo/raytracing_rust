use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Point;

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Rc<dyn Hittable>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Rc<dyn Hittable>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }

    pub fn center(&self) -> Point {
        Point::new(self.center.x(), self.center.y(), self.center.z())
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // find nearest root that lies in acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), root, Rc::clone(&self.mat), false);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
