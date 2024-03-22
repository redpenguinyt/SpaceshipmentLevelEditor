use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};
use std::{fs::File, io::Read, process};

mod app_state;
pub use app_state::AppState;

mod simulation;
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F};

pub struct Context {
    pub state: AppState,
    pub launch_strength: f64,
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub simulation: Simulation,
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
            launch_strength: 2.0,
            player: Player::from_nums(&nums[0..3]),
            target: Target::from_nums(&nums[3..6]),
            planets: (0..nums[6] as usize)
                .map(|i| Planet::from_nums(&nums[i * 3 + 7..i * 3 + 10]))
                .collect(),
            simulation: Simulation::empty(),
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
                AppState::Flying,
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

            (AppState::Aiming, Event::MouseWheel { y, .. }) => {
                self.launch_strength += *y as f64 / 10.0;
                self.launch_strength = self.launch_strength.clamp(1.0, 3.0);
            }

            _ => (),
        }
    }

    pub fn tick(&mut self) {
        if matches!(self.state, AppState::Flying) {
            match self.simulation.tick() {
                Some(SimulationEvent::Crashed) => {
                    println!("you crashed!");
                    process::exit(0);
                }
                Some(SimulationEvent::Won) => {
                    println!("you won!");
                    process::exit(0);
                }
                _ => (),
            };
        }
    }
}
