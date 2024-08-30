use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Write},
};

use super::{Planet, Player, SelectedBody, Selection, Target, Vec2F, Wall, WallEnd};

fn pop_or_eof(nums: &mut Vec<f64>) -> Result<f64, String> {
    nums.pop()
        .map_or_else(|| Err(String::from("Reached end of file early")), Ok)
}

pub struct LevelData {
    pub level_position: (i32, i32),
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub walls: Vec<Wall>,
}

impl LevelData {
    pub fn load(filepath: &str) -> Result<Self, String> {
        let mut file = File::open(filepath).map_err(|_| String::from("Failed to open file"))?;
        let mut text = String::new();
        file.read_to_string(&mut text)
            .map_err(|_| String::from("File is not valid UTF-8"))?;

        let mut clean_text = String::new();
        // Ignore comments
        let mut chars = text.chars();
        loop {
            let Some(c) = chars.next() else {
                break;
            };

            if c == '#' {
                chars.find(|ch| *ch == '\n');
            } else {
                clean_text.push(c);
            }
        }
        clean_text = clean_text.replace('\n', " ");

        let r_nums = clean_text
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::parse::<f64>);

        let mut nums = Vec::new();
        for num in r_nums {
            nums.push(num.map_err(|_| format!("Found non-number in {filepath}"))?);
        }

        nums.reverse();

        let level_position = (pop_or_eof(&mut nums)? as i32, pop_or_eof(&mut nums)? as i32);

        let player = Player::new(Vec2F::new(pop_or_eof(&mut nums)?, pop_or_eof(&mut nums)?));

        let target = Target::from_nums(&[
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
        ]);

        let mut planets = Vec::new();
        for _ in 0..nums.pop().unwrap_or_default() as usize {
            planets.push(Planet::from_nums(&[
                pop_or_eof(&mut nums)?,
                pop_or_eof(&mut nums)?,
                pop_or_eof(&mut nums)?,
            ]));
        }

        let mut walls = Vec::new();
        for _ in 0..nums.pop().unwrap_or_default() as usize {
            walls.push(Wall::from_nums(&[
                pop_or_eof(&mut nums)?,
                pop_or_eof(&mut nums)?,
                pop_or_eof(&mut nums)?,
                pop_or_eof(&mut nums)?,
            ]));
        }

        Ok(Self {
            level_position,
            player,
            target,
            planets,
            walls,
        })
    }

    pub fn save(&self, filepath: &str) -> Result<(), String> {
        let mut buffer = BufWriter::new(File::create(filepath).map_err(|e| e.to_string())?);

        let level_bytes: Vec<u8> = self.to_string().bytes().collect();

        buffer.write_all(&level_bytes).map_err(|e| e.to_string())?;
        buffer.flush().map_err(|e| e.to_string())
    }

    pub fn move_selection(&mut self, selected_body: SelectedBody, movement: Vec2F) {
        match selected_body {
            SelectedBody::Player => self.player.pos += movement,
            SelectedBody::Planet(i) => self.planets[i].pos += movement,
            SelectedBody::Target => self.target.pos += movement,
            SelectedBody::Wall(i, WallEnd::Beginning) => self.walls[i].pos1 += movement,
            SelectedBody::Wall(i, WallEnd::End) => self.walls[i].pos2 += movement,
            SelectedBody::None => (),
        };
    }

    pub fn resize_selection(&mut self, edit_selection: Selection, change: f64) {
        match edit_selection.body {
            SelectedBody::Target => self.target.change_size(change * 0.1),
            SelectedBody::Planet(i) => self.planets[i].change_size(change * 0.1),

            SelectedBody::None => {
                // Try target
                let distance_to_target =
                    (self.target.pos - edit_selection.last_mouse_pos).magnitude();

                if distance_to_target < self.target.size + 2.0 {
                    self.target.change_size(change * 0.1);
                }

                // Try planets
                for (i, planet) in self.planets.clone().into_iter().enumerate() {
                    let distance_to_planet =
                        (planet.pos - edit_selection.last_mouse_pos).magnitude();

                    if distance_to_planet < planet.mass.abs() / 12.0 {
                        self.planets[i].change_size(change * 0.1);
                        break;
                    }
                }
            }

            _ => (),
        };
    }
}

impl Display for LevelData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.level_position.0, self.level_position.1)?;

        writeln!(f, "{} {}", self.player.pos.x, self.player.pos.y)?;
        writeln!(f, "{} {}", self.target.size.round(), self.target.pos)?;

        writeln!(f, "{}", self.planets.len())?;
        for planet in &self.planets {
            writeln!(f, "{} {}", planet.mass.round(), planet.pos)?;
        }

        writeln!(f, "{}", self.walls.len())?;
        for wall in &self.walls {
            writeln!(f, "{} {}", wall.pos1, wall.pos2)?;
        }

        Ok(())
    }
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            level_position: (0, 0),
            player: Player::new(Vec2F::new(50.0, 120.0)),
            target: Target::new(20.0, Vec2F::new(330.0, 120.0)),
            planets: vec![Planet::new(400.0, Vec2F::new(200.0, 120.0))],
            walls: Vec::new(),
        }
    }
}
