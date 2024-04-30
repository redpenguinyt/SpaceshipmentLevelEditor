use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Planet {
    pub mass: f64,
    pub pos: Vec2F,
}

impl Planet {
    pub const fn new(mass: f64, pos: Vec2F) -> Self {
        Self { mass, pos }
    }

    pub const fn from_nums(nums: &[f64]) -> Self {
        assert!(
            nums.len() == 3,
            "Wrong number of items passed to `Player::from_nums`"
        );

        Self::new(nums[0], Vec2F::new(nums[1], nums[2]))
    }

    pub fn change_size(&mut self, change: f64) {
        if self.mass.is_sign_positive() {
            self.mass *= 1.0 + change;
            self.mass = self.mass.min(12000.0); // planets larger than 12000 look funky

            if self.mass < 50.0 {
                self.mass = -50.0;
            } else {
                self.mass = self.mass.max(50.0);
            }
        } else {
            self.mass *= 1.0 - change;
            self.mass = self.mass.min(-12000.0);

            if self.mass > -50.0 {
                self.mass = 50.0;
            } else {
                self.mass = self.mass.min(-50.0);
            }
        }
    }
}
