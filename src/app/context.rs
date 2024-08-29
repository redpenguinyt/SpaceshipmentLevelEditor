mod app_state;
pub use app_state::AppState;

mod event;
mod global_keybinds;

mod level_data;
pub use level_data::LevelData;

mod save_load;
use save_load::{generate_new_level_path, load_level, save_level, SaveMethod};

mod selection;
pub use selection::{SelectedBody, Selection, WallEnd};

mod simulation;
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F, Wall};

pub struct Context {
    pub state: AppState,
    pub level_path: String,
    pub level_data: LevelData,
    pub simulation: Simulation,
    pub edit_selection: Selection,
    pub show_hints: bool,
    pub show_background_image: bool,
}

impl Context {
    pub fn new() -> Self {
        println!("Created new file");

        Self {
            state: AppState::Editing,
            level_path: String::from("new level"),
            level_data: LevelData::default(),
            simulation: Simulation::empty(),
            edit_selection: Selection::new(),
            show_hints: false,
            show_background_image: true,
        }
    }

    pub fn load(&mut self, filepath: &str) -> Result<(), String> {
        self.level_path = String::from(filepath);
        self.level_data = load_level(filepath)?;

        println!("Loaded level {filepath}");

        Ok(())
    }

    pub fn save(&mut self, method: SaveMethod) -> Result<(), String> {
        match method {
            SaveMethod::ToCurrentFile => (),
            SaveMethod::Incremental => self.level_path = generate_new_level_path(&self.level_path)?,
            SaveMethod::As(path) => self.level_path = path,
        };

        save_level(&self.level_path, &self.level_data)?;

        Ok(())
    }

    pub fn tick(&mut self) {
        if matches!(self.state, AppState::Flying) {
            if let Some(simulation_event) = self.simulation.tick() {
                self.state = AppState::GameOver(simulation_event);
            };
        }
    }
}
