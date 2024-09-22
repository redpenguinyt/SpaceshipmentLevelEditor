use std::f64::consts;
use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2F,
    pub velocity: Vec2F,
}

impl Player {
    pub const fn new(pos: Vec2F) -> Self {
        Self {
            pos,
            velocity: Vec2F::new(consts::FRAC_1_SQRT_2, consts::FRAC_1_SQRT_2),
        }
    }

    pub fn aim_at(&mut self, target: Vec2F) {
        let distance_to_other = target - self.pos;

        let normalised = distance_to_other.normalised();

        let power = (distance_to_other.magnitude() - 30.0) / 30.0;
        let clamped_power = power.clamp(1.0, 3.0);

        self.velocity = normalised * clamped_power;
    }

    /// Change the angle of the velocity by `change` degrees
    pub fn change_angle(&mut self, change: f64) {
        let angle = self.velocity.y.atan2(self.velocity.x) + change.to_radians();
        let power = self.velocity.magnitude();

        self.velocity = Vec2F::new(angle.cos(), angle.sin()) * power;
    }

    /// Change the magnitude of the velocity by `change`
    pub fn change_power(&mut self, change: f64) {
        let normal = self.velocity.normalised();

        let power = self.velocity.magnitude() + change;
        let clamped_power = power.clamp(1.0, 3.0);

        self.velocity = normal * clamped_power;
    }
}
