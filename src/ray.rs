use super::vector::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn zero() -> Ray {
        Ray {
            origin: Point3::zero(),
            direction: Vec3::zero()
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
