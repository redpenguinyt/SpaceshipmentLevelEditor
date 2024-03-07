use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Target {
	pub size: f64,
	pub position: Vec2F,
}

impl Target {
	pub fn new(size: f64, position: Vec2F) -> Self {
		Self { size, position }
	}
}