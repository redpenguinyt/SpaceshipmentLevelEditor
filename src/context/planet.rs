use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Planet {
	pub mass: f64,
	pub position: Vec2F,
}

impl Planet {
	pub const fn new(mass: f64, position: Vec2F) -> Self {
		Self { mass, position }
	}
}