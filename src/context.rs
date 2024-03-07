mod planet;

use std::{fs::File, io::Read, process};

pub use planet::Planet;

mod player;
pub use player::Player;

mod target;
pub use target::Target;

mod vec2f;
pub use vec2f::Vec2F;

// const G: f64 = 6.67430e-11;
const G: f64 = 0.55;

pub struct Context {
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
}

impl Context {
    pub fn build(filepath: &str) -> Self {
        let mut file = File::open(filepath).expect("Could not load file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Could not read file");

        let nums: Vec<f64> = text
            .replace('\n', " ")
            .split(' ')
            .map(str::parse::<f64>)
            .map(|r| r.expect("Could not parse to f64"))
            .collect();

        Self {
            player: Player::new(nums[0], Vec2F::new(nums[1], nums[2])),
            target: Target::new(nums[3], Vec2F::new(nums[4], nums[5])),
            planets: (0..nums[6] as usize)
                .map(|i| {
                    Planet::new(
                        nums[i * 3 + 7],
                        Vec2F::new(nums[i * 3 + 8], nums[i * 3 + 9]),
                    )
                })
                .collect(),
        }
    }

    /// Return true if the player crashes
    fn gravitate_player(&mut self) -> bool {
        for planet in &self.planets {
            let distance = planet.position - self.player.position;
            let magnitude = distance.x.powi(2) + distance.y.powi(2);

            let force = G * self.player.mass * planet.mass / magnitude;

            let acceleration = force / self.player.mass;

            let angle = (distance.y).atan2(distance.x);

            self.player.acceleration +=
                Vec2F::new(acceleration * angle.cos(), acceleration * angle.sin());

            if magnitude < planet.mass.powi(2) / 144.0 {
                return true;
            }
        }

        self.player.position += self.player.acceleration;

        false
    }

    fn is_touching_target(&self) -> bool {
        let vecdistance = self.target.position - self.player.position;

        let distance = vecdistance.x * vecdistance.x + vecdistance.y * vecdistance.y;

        distance < self.target.size.powi(2)
    }

    pub fn tick(&mut self) {
        if self.gravitate_player() {
            println!("you crashed!");
            process::exit(0);
        }

        if self.is_touching_target() {
            println!("level complete!");
            process::exit(0);
        }
    }
}
