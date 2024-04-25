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
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F, Wall};

use self::selection::WallEnd;

pub struct Context {
    pub state: AppState,
    pub level_path: String,
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub walls: Vec<Wall>,
    pub simulation: Simulation,
    pub edit_selection: Selection,
    pub show_hints: bool,
    pub show_background_image: bool,
}

impl Context {
    pub fn build(filepath: &str) -> Result<Self, String> {
        let (player, target, planets, walls) = load_level(filepath)?;

        println!("Loaded {filepath}");

        Ok(Self {
            state: AppState::Editing,
            level_path: String::from(filepath),
            player,
            target,
            planets,
            walls,
            simulation: Simulation::empty(),
            edit_selection: Selection::new(),
            show_hints: true,
            show_background_image: true,
        })
    }

    pub fn save(&mut self, method: SaveMethod) -> Result<(), String> {
        match method {
            SaveMethod::ToCurrentFile => (),
            SaveMethod::Incremental => self.level_path = generate_new_level_path(&self.level_path)?,
            SaveMethod::As(path) => self.level_path = path,
        };

        save_level(
            &self.level_path,
            &self.player,
            &self.target,
            &self.planets,
            &self.walls,
        )?;

        Ok(())
    }

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
                    self.walls.clone(),
                );
            }

            // Aim with the mouse
            (AppState::Aiming, Event::MouseMotion { x, y, .. }) => {
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

            (
                AppState::Flying,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                },
            ) => {
                self.simulation.playing = !self.simulation.playing;
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
                self.simulation.speed = *keycode as u32 - 48;
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

    fn move_selected_body(&mut self, movement: Vec2F) {
        match self.edit_selection.body {
            SelectedBody::Player => self.player.pos += movement,
            SelectedBody::Planet(i) => self.planets[i].pos += movement,
            SelectedBody::Target => self.target.pos += movement,
            SelectedBody::Wall(i, WallEnd::Beginning) => self.walls[i].pos1 += movement,
            SelectedBody::Wall(i, WallEnd::End) => self.walls[i].pos2 += movement,
            SelectedBody::None => (),
        };
    }

    fn try_select_any_body(&mut self, mouse_pos: Vec2F) {
        // Try Player
        if self
            .edit_selection
            .try_select(mouse_pos, SelectedBody::Player, self.player.pos, 14.0)
        {
            return;
        }

        // Try Target
        if self.edit_selection.try_select(
            mouse_pos,
            SelectedBody::Target,
            self.target.pos,
            self.target.size + 2.0,
        ) {
            return;
        }

        // Try walls
        for (i, wall) in self.walls.iter().enumerate() {
            if self.edit_selection.try_select(
                mouse_pos,
                SelectedBody::Wall(i, WallEnd::Beginning),
                wall.pos1,
                8.0,
            ) {
                return;
            }

            if self.edit_selection.try_select(
                mouse_pos,
                SelectedBody::Wall(i, WallEnd::End),
                wall.pos2,
                8.0,
            ) {
                return;
            }
        }

        // Try planets
        for (i, planet) in self.planets.iter().enumerate() {
            if self.edit_selection.try_select(
                mouse_pos,
                SelectedBody::Planet(i),
                planet.pos,
                planet.mass.abs() / 12.0,
            ) {
                return;
            }
        }
    }

    fn change_body_size(&mut self, change: f64) {
        match self.edit_selection.body {
            SelectedBody::Target => {
                self.change_target_size(change * 0.1);
            }

            SelectedBody::Planet(i) => {
                self.change_planet_size(i, change * 0.1);
            }

            SelectedBody::Wall(_, _) | SelectedBody::Player => (),

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
            if let Some(simulation_event) = self.simulation.tick() {
                self.state = AppState::GameOver(simulation_event);
            };
        }
    }
}
