use crate::{hittable::HitRecord, ray::Ray, vector::{Color, random_unit_vec}};

pub trait Material {
    /// Returns an Option of an attenuation color and a scattered ray
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vec();

        // catch bad scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(&rec.p, &scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
