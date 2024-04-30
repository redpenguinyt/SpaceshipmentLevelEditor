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
                let mouse_pos = Vec2F::new(*x as f64, *y as f64);

                self.try_select_any_body(mouse_pos);
            }

            Event::MouseMotion { x, y, .. } => {
                let mouse_pos = Vec2F::new(*x as f64, *y as f64);
                let mouse_movement = mouse_pos - self.edit_selection.last_mouse_pos;

                self.move_selected_body(mouse_movement);

                self.edit_selection.last_mouse_pos = mouse_pos;
            }

            Event::MouseWheel { y, .. } => {
                self.change_body_size(*y as f64);
            }

            // Move selected body with arrow keys
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.move_selected_body(Vec2F::new(0.0, -1.0));
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.move_selected_body(Vec2F::new(0.0, 1.0));
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.move_selected_body(Vec2F::new(-1.0, 0.0));
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.move_selected_body(Vec2F::new(1.0, 0.0));
            }

            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => self.edit_selection.deselect(),

            Event::KeyDown {
                keycode: Some(Keycode::A | Keycode::N),
                ..
            } => {
                self.planets
                    .push(Planet::new(400.0, self.edit_selection.last_mouse_pos));

                self.edit_selection.body = SelectedBody::Planet(self.planets.len() - 1);
            }

            Event::KeyDown {
                keycode: Some(Keycode::W | Keycode::L),
                ..
            } => {
                self.walls.push(Wall::new(
                    self.edit_selection.last_mouse_pos,
                    self.edit_selection.last_mouse_pos,
                ));

                self.edit_selection.body =
                    SelectedBody::Wall(self.walls.len() - 1, WallEnd::Beginning);
            }

            Event::KeyDown {
                keycode: Some(Keycode::H),
                ..
            } => self.edit_selection.toggle_grab_indicators(),

            Event::KeyDown {
                keycode: Some(Keycode::D | Keycode::Backspace | Keycode::X),
                ..
            } => match self.edit_selection.body {
                SelectedBody::Planet(i) => {
                    self.planets.remove(i);

                    self.edit_selection.body = SelectedBody::None;
                }

                SelectedBody::Wall(i, _) => {
                    self.walls.remove(i);

                    self.edit_selection.body = SelectedBody::None;
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
                self.simulation.push(
                    self.player.clone(),
                    self.target.clone(),
                    self.planets.clone(),
                    self.walls.clone(),
                );
            }

            // Aim with the mouse
            Event::MouseMotion { x, y, .. } => {
                if matches!(self.state, AppState::Aiming) {
                    let distance_to_mouse =
                        Vec2F::new(*x as f64 - self.player.pos.x, *y as f64 - self.player.pos.y);

                    let mut normalised = distance_to_mouse / distance_to_mouse.magnitude();

                    if normalised.x.is_nan() || normalised.y.is_nan() {
                        normalised = Vec2F::new(1.0, 0.0);
                    }

                    let launch_strength = (distance_to_mouse.magnitude() - 30.0) / 30.0;
                    let clamped_launch_strength = launch_strength.clamp(1.0, 3.0);

                    self.player.velocity = normalised * clamped_launch_strength;
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
