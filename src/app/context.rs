mod app_state;
pub use app_state::AppState;

mod event;
mod global_keybinds;

mod save_load;
use save_load::{generate_new_level_path, load_level, save_level};
pub use save_load::SaveMethod;

mod selection;
pub use selection::{SelectedBody, Selection, WallEnd};

mod simulation;
pub use simulation::{Event as SimulationEvent, Planet, Player, Simulation, Target, Vec2F, Wall};

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
    pub fn new() -> Self {
        println!("Created new file");

        Self {
            state: AppState::Editing,
            level_path: String::from("new level"),
            player: Player::new(Vec2F::new(50.0, 120.0)),
            target: Target::new(20.0, Vec2F::new(330.0, 120.0)),
            planets: vec![Planet::new(400.0, Vec2F::new(200.0, 120.0))],
            walls: Vec::new(),
            simulation: Simulation::empty(),
            edit_selection: Selection::new(),
            show_hints: false,
            show_background_image: true,
        }
    }

    pub fn load(&mut self, filepath: &str) -> Result<(), String> {
        let (player, target, planets, walls) = load_level(filepath)?;

        self.level_path = String::from(filepath);
        self.player = player;
        self.target = target;
        self.planets = planets;
        self.walls = walls;

        println!("Loaded level {filepath}");

        Ok(())
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
                self.target.change_size(change * 0.1);
            }

            SelectedBody::Planet(i) => {
                self.planets[i].change_size(change * 0.1);
            }

            SelectedBody::None => {
                // Try target
                let distance_to_target =
                    (self.target.pos - self.edit_selection.last_mouse_pos).magnitude();

                if distance_to_target < self.target.size + 2.0 {
                    self.target.change_size(change * 0.1);
                }

                // Try planets
                for (i, planet) in self.planets.clone().into_iter().enumerate() {
                    let distance_to_planet =
                        (planet.pos - self.edit_selection.last_mouse_pos).magnitude();

                    if distance_to_planet < planet.mass.abs() / 12.0 {
                        self.planets[i].change_size(change * 0.1);
                        break;
                    }
                }
            }

            _ => (),
        };
    }

    pub fn tick(&mut self) {
        if matches!(self.state, AppState::Flying) {
            if let Some(simulation_event) = self.simulation.tick() {
                self.state = AppState::GameOver(simulation_event);
            };
        }
    }
}
