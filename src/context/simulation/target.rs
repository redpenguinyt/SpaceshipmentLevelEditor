use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Target {
    pub size: f64,
    pub position: Vec2F,
}

impl Target {
    pub const fn new(size: f64, position: Vec2F) -> Self {
        Self { size, position }
    }

    pub const fn from_nums(nums: &[f64]) -> Self {
        assert!(
            nums.len() == 3,
            "Wrong number of items passed to `Player::from_nums`"
        );

        Self::new(nums[0], Vec2F::new(nums[1], nums[2]))
    }
}
