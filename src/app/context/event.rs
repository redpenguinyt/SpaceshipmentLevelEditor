use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};

use super::{AppState, Planet, SelectedBody, Vec2F, Wall, WallEnd};

/// Event methods
impl super::Context {
    pub fn event(&mut self, event: &Event) {
        match (self.state, event) {
            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                },
            ) => self.show_hints = !self.show_hints,

            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                },
            ) => self.show_background_image = !self.show_background_image,

            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                },
            ) => self.state.toggle(),

            (
                AppState::Flying | AppState::GameOver(_),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                },
            )
            | (
                AppState::Editing,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                },
            ) => self.state = AppState::Aiming,

            (AppState::Editing, _) => self.edit_event(event),

            (AppState::Aiming, _) => self.aim_event(event),

            (AppState::Flying, _) => self.fly_event(event),

            _ => (),
        }
    }

    fn edit_event(&mut self, event: &Event) {
        match event {
            // Moving elements around
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let mouse_pos = Vec2F::from_mouse_pos(*x, *y);

                self.edit_selection.try_select(&self.level_data, mouse_pos);
            }

            Event::MouseMotion { x, y, .. } => {
                let mouse_pos = Vec2F::from_mouse_pos(*x, *y);

                self.level_data.move_selection(
                    self.edit_selection.body,
                    mouse_pos - self.edit_selection.last_mouse_pos,
                );

                self.edit_selection.last_mouse_pos = mouse_pos;
            }

            Event::MouseWheel { y, .. } => {
                self.level_data.resize_selection(self.edit_selection, *y);
            }

            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => self.edit_selection.deselect(),

            Event::KeyDown {
                keycode: Some(Keycode::D),
                keymod,
                ..
            } if keymod.contains(Mod::LCTRLMOD) => {
                if let SelectedBody::Planet(i) = self.edit_selection.body {
                    self.level_data
                        .planets
                        .push(self.level_data.planets[i].clone());
                }
            }

            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                // Move selected body with arrow keys
                Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                    self.level_data.move_selection(
                        self.edit_selection.body,
                        Vec2F::try_from(keycode).unwrap_or_else(|_| unreachable!()),
                    );
                }

                Keycode::A | Keycode::N => {
                    self.level_data
                        .planets
                        .push(Planet::new(400.0, self.edit_selection.last_mouse_pos));

                    self.edit_selection.body =
                        SelectedBody::Planet(self.level_data.planets.len() - 1);
                }

                Keycode::W | Keycode::L => {
                    self.level_data.walls.push(Wall::new(
                        self.edit_selection.last_mouse_pos,
                        self.edit_selection.last_mouse_pos,
                    ));

                    self.edit_selection.body =
                        SelectedBody::Wall(self.level_data.walls.len() - 1, WallEnd::Beginning);
                }

                Keycode::H => self.edit_selection.toggle_grab_indicators(),

                Keycode::D | Keycode::Backspace | Keycode::X => match self.edit_selection.body {
                    SelectedBody::Planet(i) => {
                        self.level_data.planets.remove(i);

                        self.edit_selection.body = SelectedBody::None;
                    }

                    SelectedBody::Wall(i, _) => {
                        self.level_data.walls.remove(i);

                        self.edit_selection.body = SelectedBody::None;
                    }

                    _ => (),
                },

                Keycode::I => {
                    if let SelectedBody::Planet(i) = self.edit_selection.body {
                        self.level_data.planets[i].mass *= -1.0;
                    }
                }

                _ => (),
            },

            _ => (),
        }
    }

    fn aim_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            }
            | Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.state = AppState::Flying;
                self.simulation.push(&self.level_data);
            }

            // Aim with the mouse
            Event::MouseMotion { x, y, .. } => {
                if matches!(self.state, AppState::Aiming) {
                    let distance_to_mouse =
                        Vec2F::from_mouse_pos(*x, *y) - self.level_data.player.pos;

                    let mut normalised = distance_to_mouse / distance_to_mouse.magnitude();

                    if normalised.x.is_nan() || normalised.y.is_nan() {
                        normalised = Vec2F::new(1.0, 0.0);
                    }

                    let launch_strength = (distance_to_mouse.magnitude() - 30.0) / 30.0;
                    let clamped_launch_strength = launch_strength.clamp(1.0, 3.0);

                    self.level_data.player.velocity = normalised * clamped_launch_strength;
                }
            }

            _ => (),
        }
    }

    fn fly_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.simulation.playing = !self.simulation.playing;
            }

            Event::KeyDown {
                keymod,
                keycode: Some(keycode),
                ..
            } if !keymod.contains(Mod::LALTMOD) && (49..=52).contains(&(*keycode as i32)) => {
                // Num1 to Num4
                self.simulation.speed = *keycode as u32 - 48;
            }

            _ => (),
        }
    }
}
