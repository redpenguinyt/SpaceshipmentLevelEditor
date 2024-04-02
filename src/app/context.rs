use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};

mod app_state;
pub use app_state::AppState;

mod save_load;
use save_load::{generate_new_level_path, load_level, save_level};
pub use save_load::{get_last_file_in_dir, SaveMethod};

mod selection;
pub use selection::{SelectedBody, Selection};

mod simulation;
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F};

pub struct Context {
    pub state: AppState,
    pub level_path: String,
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub simulation: Simulation,
    pub edit_selection: Selection,
    pub simulation_speed: u32,
}

impl Context {
    pub fn build(filepath: &str) -> Self {
        let (player, target, planets) = load_level(filepath);

        Self {
            state: AppState::Editing,
            level_path: String::from(filepath),
            player,
            target,
            planets,
            simulation: Simulation::empty(),
            edit_selection: Selection::new(),
            simulation_speed: 1,
        }
    }

    pub fn save(&mut self, method: SaveMethod) -> Result<(), String> {
        match method {
            SaveMethod::ToCurrentFile => (),
            SaveMethod::Incremental => self.level_path = generate_new_level_path(&self.level_path),
            SaveMethod::As(path) => self.level_path = path,
        };

        save_level(&self.level_path, &self.player, &self.target, &self.planets)?;

        Ok(())
    }

    pub fn event(&mut self, event: &Event) {
        if matches!(
            event,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
        ) {
            self.state.toggle();
        }

        match (self.state, event) {
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

            (
                AppState::Aiming,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                }
                | Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                },
            ) => {
                self.state = AppState::Flying;
                self.simulation.push(
                    self.player.clone(),
                    self.target.clone(),
                    self.planets.clone(),
                );
            }

            // Aim with the mouse
            (AppState::Aiming, Event::MouseMotion { x, y, .. }) => {
                if matches!(self.state, AppState::Aiming) {
                    let distance_to_mouse =
                        Vec2F::new(*x as f64 - self.player.pos.x, *y as f64 - self.player.pos.y);

                    let normalised = distance_to_mouse / distance_to_mouse.magnitude();

                    let clamped_distance = distance_to_mouse.magnitude().clamp(30.0, 90.0);
                    let launch_strength = clamped_distance / 30.0;

                    self.player.acceleration = normalised * launch_strength;

                    // display launch strength while aiming
                }
            }

            (
                AppState::Flying,
                Event::KeyDown {
                    keymod,
                    keycode: Some(keycode),
                    ..
                },
            ) if !keymod.contains(Mod::LALTMOD) && (49..=52).contains(&(*keycode as i32)) => {
                // Num1 to Num4
                self.simulation_speed = *keycode as u32 - 48;
            }

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
            } => 'body_select: {
                let mouse_pos = Vec2F::new(*x as f64, *y as f64);

                // Try Player
                if self.edit_selection.try_select(
                    mouse_pos,
                    SelectedBody::Player,
                    self.player.pos,
                    14.0,
                ) {
                    break 'body_select;
                }

                // Try Target
                if self.edit_selection.try_select(
                    mouse_pos,
                    SelectedBody::Target,
                    self.target.pos,
                    self.target.size + 2.0,
                ) {
                    break 'body_select;
                }

                // Try planets
                for (i, planet) in self.planets.iter().enumerate() {
                    if self.edit_selection.try_select(
                        mouse_pos,
                        SelectedBody::Planet(i),
                        planet.pos,
                        planet.mass.abs() / 12.0,
                    ) {
                        break 'body_select;
                    }
                }
            }

            Event::MouseMotion { x, y, .. } => {
                let mouse_pos = Vec2F::new(*x as f64, *y as f64);
                let mouse_movement = mouse_pos - self.edit_selection.last_mouse_pos;

                match self.edit_selection.body {
                    SelectedBody::Player => self.player.pos += mouse_movement,
                    SelectedBody::Planet(i) => self.planets[i].pos += mouse_movement,
                    SelectedBody::Target => self.target.pos += mouse_movement,
                    SelectedBody::None => (),
                };

                self.edit_selection.last_mouse_pos = mouse_pos;
            }

            Event::MouseWheel { y, .. } => {
                self.change_body_size(*y as f64);
            }

            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.change_body_size(0.2);
            }

            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.change_body_size(-0.2);
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
                keycode: Some(Keycode::D | Keycode::Backspace | Keycode::X),
                ..
            } => {
                if let SelectedBody::Planet(i) = self.edit_selection.body {
                    self.planets.remove(i);

                    self.edit_selection.body = SelectedBody::None;
                }
            }

            _ => (),
        }
    }

    fn change_body_size(&mut self, change: f64) {
        match self.edit_selection.body {
            SelectedBody::Player => (),

            SelectedBody::Target => {
                self.change_target_size(change * 0.1);
            }

            SelectedBody::Planet(i) => {
                self.change_planet_size(i, change * 0.1);
            }

            SelectedBody::None => {
                // Try target
                let distance_to_target =
                    (self.target.pos - self.edit_selection.last_mouse_pos).magnitude();

                if distance_to_target < self.target.size + 2.0 {
                    self.change_target_size(change * 0.1);
                }

                // Try planets
                for (i, planet) in self.planets.clone().into_iter().enumerate() {
                    let distance_to_planet =
                        (planet.pos - self.edit_selection.last_mouse_pos).magnitude();

                    if distance_to_planet < planet.mass.abs() / 12.0 {
                        self.change_planet_size(i, change * 0.1);
                    }
                }
            }
        };
    }

    fn change_target_size(&mut self, change: f64) {
        self.target.size *= 1.0 + change;
        self.target.size = self.target.size.max(5.0);
    }

    fn change_planet_size(&mut self, i: usize, change: f64) {
        if self.planets[i].mass.is_sign_positive() {
            self.planets[i].mass *= 1.0 + change;

            if self.planets[i].mass < 50.0 {
                self.planets[i].mass = -50.0;
            } else {
                self.planets[i].mass = self.planets[i].mass.max(50.0);
            }
        } else {
            self.planets[i].mass *= 1.0 - change;

            if self.planets[i].mass > -50.0 {
                self.planets[i].mass = 50.0;
            } else {
                self.planets[i].mass = self.planets[i].mass.min(-50.0);
            }
        }
    }

    pub fn tick(&mut self) {
        if matches!(self.state, AppState::Flying) {
            for _ in 0..self.simulation_speed {
                if let Some(simulation_event) = self.simulation.tick() {
                    self.state = AppState::GameOver(simulation_event);
                    break;
                };
            }
        }
    }
}
