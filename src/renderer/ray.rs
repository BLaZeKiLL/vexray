use crate::utils::math::{Point3, Vec3, Color3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}

pub fn ray_color(ray: &Ray) -> Color3 {
    let direction = ray.direction().normalized();
    let t = 0.5 * (direction.y() + 1.0);
	(1.0 - t) * Color3::one() + t * Color3::new(0.5, 0.7, 1.0)
}
