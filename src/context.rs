use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};
use std::{fs::File, io::Read};

mod app_state;
pub use app_state::AppState;

mod selection;
pub use selection::{SelectedBody, Selection};

mod simulation;
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F};

pub struct Context {
    pub state: AppState,
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub simulation: Simulation,
    pub edit_selection: Selection,
    pub simulation_speed: u32,
}

impl Context {
    pub fn build(filepath: &str) -> Self {
        let mut file = File::open(filepath).expect("Could not load file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Could not read file");

        let nums: Vec<f64> = text
            .replace('\n', " ")
            .split(' ')
            .filter(|s| !s.is_empty())
            .skip(2)
            .map(str::parse::<f64>)
            .map(|r| r.expect("Could not parse to f64"))
            .collect();

        Self {
            state: AppState::Editing,
            player: Player::from_nums(&nums[0..3]),
            target: Target::from_nums(&nums[3..6]),
            planets: (0..nums[6] as usize)
                .map(|i| Planet::from_nums(&nums[i * 3 + 7..i * 3 + 10]))
                .collect(),
            simulation: Simulation::empty(),
            edit_selection: Selection::new(),
            simulation_speed: 1,
        }
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
                    keymod: Mod::NOMOD,
                    keycode: Some(keycode),
                    ..
                },
            ) => {
                let keynum = *keycode as i32;
                // Num1 to Num4
                if (49..=52).contains(&keynum) {
                    self.simulation_speed = keynum as u32 - 48;
                }
            }

            _ => (),
        }
    }

    pub fn edit_event(&mut self, event: &Event) {
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
                    17.0,
                ) {
                    break 'body_select;
                }

                // Try planets
                for (i, planet) in self.planets.iter().enumerate() {
                    if self.edit_selection.try_select(
                        mouse_pos,
                        SelectedBody::Planet(i),
                        planet.pos,
                        planet.mass / 12.0,
                    ) {
                        break 'body_select;
                    }
                }
            }

            Event::MouseMotion { x, y, .. } => 'mouse_move: {
                let selected_body_positon = match self.edit_selection.body {
                    SelectedBody::Player => &mut self.player.pos,
                    SelectedBody::Planet(i) => &mut self.planets[i].pos,
                    SelectedBody::Target => &mut self.target.pos,
                    SelectedBody::None => break 'mouse_move,
                };

                let mouse_pos = Vec2F::new(*x as f64, *y as f64);
                let mouse_movement = mouse_pos - self.edit_selection.last_mouse_position;

                *selected_body_positon += mouse_movement;

                self.edit_selection.last_mouse_position = mouse_pos;
            }

            Event::MouseWheel { y, .. } => 'mouse_scroll: {
                match self.edit_selection.body {
                    SelectedBody::Planet(i) => {
                        self.planets[i].mass *= (*y as f64).mul_add(0.1, 1.0);
                        self.planets[i].mass = self.planets[i].mass.max(50.0);
                    }

                    SelectedBody::Target => {
                        self.target.size *= (*y as f64).mul_add(0.1, 1.0);
                        self.target.size = self.target.size.max(5.0);
                    }

                    _ => break 'mouse_scroll,
                };
            }

            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.edit_selection.body = SelectedBody::None;
            }

            _ => (),
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
