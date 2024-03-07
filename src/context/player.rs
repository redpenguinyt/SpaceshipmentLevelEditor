use super::Vec2F;

pub struct Player {
	pub mass: f64,
	pub position: Vec2F,
	pub acceleration: Vec2F,
}

impl Player {
	pub const fn new(mass: f64, position: Vec2F) -> Self {
		Self { mass, position, acceleration: Vec2F::ZERO }
	}
}