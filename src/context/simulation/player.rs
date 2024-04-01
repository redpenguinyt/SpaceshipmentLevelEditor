use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2F,
    pub acceleration: Vec2F,
}

impl Player {
    pub const fn new(pos: Vec2F) -> Self {
        Self {
            pos,
            acceleration: Vec2F::ZERO,
        }
    }
}
