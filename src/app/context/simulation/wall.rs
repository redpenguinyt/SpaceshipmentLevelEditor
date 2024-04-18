use super::Vec2F;

fn ccw(a: Vec2F, b: Vec2F, c: Vec2F) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub pos1: Vec2F,
    pub pos2: Vec2F,
}

impl Wall {
    pub const fn new(pos1: Vec2F, pos2: Vec2F) -> Self {
        Self { pos1, pos2 }
    }

    pub const fn from_nums(nums: &[f64]) -> Self {
        assert!(
            nums.len() == 4,
            "Wrong number of items passed to `Player::from_nums`"
        );

        Self::new(Vec2F::new(nums[0], nums[1]), Vec2F::new(nums[2], nums[3]))
    }

    pub fn intersects(&self, other: &Self) -> bool {
        // return ccw(ax, ay, cx, cy, dx, dy) != ccw(bx, by, cx, cy, dx, dy) &&
        // ccw(ax, ay, bx, by, cx, cy) != ccw(ax, ay, bx, by, dx, dy);
        ccw(self.pos1, other.pos1, other.pos2) != ccw(self.pos2, other.pos1, other.pos2)
            && ccw(self.pos1, self.pos2, other.pos1) != ccw(self.pos1, self.pos2, other.pos2)
    }
}
