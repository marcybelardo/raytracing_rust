use rand::prelude::*;

use crate::{
    degrees_to_radians,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vector::{Color, Point, Vector3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vector3,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point,
    pixelzero_loc: Point,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        self.init();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", (self.image_height - j));

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                println!("{}", write_color(self.pixel_samples_scale * pixel_color));
            }
        }

        eprintln!("\rDone!              \n");
    }

    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixelzero_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let r = self.get_ray(self.image_width / 2, self.image_height / 2);
        eprintln!("origin = {:?}", r.origin());
        eprintln!("dir    = {:?}", r.direction());
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel at location i, j
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixelzero_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

/// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
fn sample_square() -> Vector3 {
    let mut rng = rand::rng();

    Vector3::new(
        rng.random_range(0.0..1.0) - 0.5,
        rng.random_range(0.0..1.0) - 0.5,
        0.0,
    )
}

fn ray_color(r: &Ray, depth: u32, world: &impl Hittable) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Color) -> String {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.000, 0.999);
    format!(
        "{} {} {}",
        (256.00 * intensity.clamp(r)) as u32,
        (256.00 * intensity.clamp(g)) as u32,
        (256.00 * intensity.clamp(b)) as u32,
    )
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point::new(0.0, 0.0, 0.0),
            lookat: Point::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            image_height: 100,
            pixel_samples_scale: 1.0,
            center: Point::new(0.0, 0.0, 0.0),
            pixelzero_loc: Point::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            w: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
