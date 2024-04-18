use super::Vec2F;

#[derive(Debug, Clone, Copy)]
pub enum WallEnd {
    Beginning,
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedBody {
    Player,
    Target,
    Planet(usize),
    Wall(usize, WallEnd),
    None,
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub body: SelectedBody,
    pub last_mouse_pos: Vec2F,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            body: SelectedBody::None,
            last_mouse_pos: Vec2F::ZERO,
        }
    }

    /// Attempt to select a body. Returns true if the body is actually selected
    pub fn try_select(
        &mut self,
        mouse_pos: Vec2F,
        body: SelectedBody,
        body_pos: Vec2F,
        body_radius: f64,
    ) -> bool {
        let distance_to_body = (body_pos - mouse_pos).magnitude();

        let is_selectable = distance_to_body < body_radius;
        if is_selectable {
            self.body = body;
            self.last_mouse_pos = mouse_pos;
        }

        is_selectable
    }

    pub fn deselect(&mut self) {
        self.body = SelectedBody::None;
    }
}
