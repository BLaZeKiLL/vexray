use std::f64::consts::PI;

pub struct Math;

impl Math {
	pub fn degrees_to_radians(degrees: f64) -> f64 {
		degrees * PI / 180.0
	}

	pub fn random_double() -> f64 {
		rand::random::<f64>()
	}

	pub fn random_double_in_range(min: f64, max: f64) -> f64 {
		min + (max - min) * Self::random_double()
	}
}

