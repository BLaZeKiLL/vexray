use super::vec3::Color3;

impl Color3 {
	pub fn black() -> Color3 {
		Color3::new(0.0, 0.0, 0.0)
	}

	pub fn white() -> Color3 {
		Color3::new(1.0, 1.0, 1.0)
	}

	pub fn red() -> Color3 {
		Color3::new(1.0, 0.0, 0.0)
	}

	pub fn green() -> Color3 {
		Color3::new(0.0, 1.0, 0.0)
	}

	pub fn blue() -> Color3 {
		Color3::new(0.0, 0.0, 1.0)
	}
}