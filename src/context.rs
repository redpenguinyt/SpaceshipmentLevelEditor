use std::{fs::File, io::Read};

mod simulation;
pub use simulation::{Planet, Player, Target, Vec2F};

use simulation::Simulation;

pub enum AppState {
    Editing,
    Aiming,
    Flying,
}

pub struct Context {
    pub state: AppState,
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
            .skip(2)
            .map(str::parse::<f64>)
            .map(|r| r.expect("Could not parse to f64"))
            .collect();

        Self {
            state: AppState::Editing,
            player: Player::from_nums(&nums[0..3]),
            target: Target::from_nums(&nums[3..6]),
            planets: (0..nums[6] as usize)
                .map(|i| {
                    Planet::from_nums(&nums[i * 3 + 7..i * 3 + 10])
                })
                .collect(),
            simulation: Simulation::empty(),
        }
    }

    pub fn start_simulation(&mut self) {
        self.state = AppState::Flying;
        self.simulation.push(
            self.player.clone(),
            self.target.clone(),
            self.planets.clone(),
        );
    }

    pub fn tick(&mut self) {
        match self.state {
            AppState::Flying => self.simulation.tick(),
            _ => (),
        }
    }
}
