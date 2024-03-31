use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Player {
    pub mass: f64,
    pub pos: Vec2F,
    pub acceleration: Vec2F,
}

impl Player {
    pub const fn new(mass: f64, pos: Vec2F) -> Self {
        Self {
            mass,
            pos,
            acceleration: Vec2F::ZERO,
        }
    }

    pub const fn from_nums(nums: &[f64]) -> Self {
        assert!(
            nums.len() == 3,
            "Wrong number of items passed to `Player::from_nums`"
        );

        Self::new(nums[0], Vec2F::new(nums[1], nums[2]))
    }
}
