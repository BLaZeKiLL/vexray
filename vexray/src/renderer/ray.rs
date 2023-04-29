use crate::{utils::vec3::{Point3, Vec3, Color3}, shapes::hit::{Hittable, HitResult}, world::World};

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

	pub fn color(&self, world: &World, depth: i32) -> Color3 {
		if depth <= 0 {
			return Color3::black();
		}

		let hit_result: HitResult = world.hit(self, 0.001, f64::MAX);

		match hit_result {
			HitResult::Fail => { // Background, could be Hdri
				let direction = self.direction().normalized();
				let blend = 0.5 * (direction.y() + 1.0);
				(1.0 - blend) * Color3::white() + blend * Color3::new(0.5, 0.7, 1.0)
			},
			HitResult::Success(hit) => { // Bounce
				let target = hit.point() + hit.normal() + Point3::random_unit_vector(); // New Diffuse
				// let target = hit.point() + Point3::random_in_hemisphere(&hit.normal()); // Old Diffuse

				let ray = Ray::new(hit.point(), target - hit.point());

				0.5 * ray.color(world, depth - 1)
			},
		}
	}
}
