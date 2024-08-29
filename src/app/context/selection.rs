use super::{LevelData, Vec2F};

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

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub body: SelectedBody,
    pub last_mouse_pos: Vec2F,
    pub show_grab_indicators: bool,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            body: SelectedBody::None,
            last_mouse_pos: Vec2F::ZERO,
            show_grab_indicators: true,
        }
    }

    pub fn toggle_grab_indicators(&mut self) {
        self.show_grab_indicators = !self.show_grab_indicators;
    }

    pub fn try_select(&mut self, level_data: &LevelData, mouse_pos: Vec2F) {
        // Try Player
        if self.try_select_body(mouse_pos, SelectedBody::Player, level_data.player.pos, 14.0) {
            return;
        }

        // Try Target
        if self.try_select_body(
            mouse_pos,
            SelectedBody::Target,
            level_data.target.pos,
            level_data.target.size + 2.0,
        ) {
            return;
        }

        // Try walls
        for (i, wall) in level_data.walls.iter().enumerate() {
            if self.try_select_body(
                mouse_pos,
                SelectedBody::Wall(i, WallEnd::Beginning),
                wall.pos1,
                8.0,
            ) {
                return;
            }

            if self.try_select_body(
                mouse_pos,
                SelectedBody::Wall(i, WallEnd::End),
                wall.pos2,
                8.0,
            ) {
                return;
            }
        }

        // Try planets
        for (i, planet) in level_data.planets.iter().enumerate() {
            if self.try_select_body(
                mouse_pos,
                SelectedBody::Planet(i),
                planet.pos,
                planet.mass.abs() / 12.0,
            ) {
                return;
            }
        }
    }

    /// Attempt to select a body. Returns true if the body is actually selected
    fn try_select_body(
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
