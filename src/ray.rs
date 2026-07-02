use crate::vector::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Self {
        Self {
            origin: Vector3::new(origin.x(), origin.y(), origin.z()),
            direction: Vector3::new(direction.x(), direction.y(), direction.z()),
        }
    }

    pub fn origin(&self) -> Vector3 {
        Vector3::new(self.origin.x(), self.origin.y(), self.origin.z())
    }

    pub fn direction(&self) -> Vector3 {
        Vector3::new(self.direction.x(), self.direction.y(), self.direction.z())
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}
