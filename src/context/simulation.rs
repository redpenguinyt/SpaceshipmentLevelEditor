mod planet;
use std::process;

pub use planet::Planet;

mod player;
pub use player::Player;

mod target;
pub use target::Target;

mod vec2f;
pub use vec2f::Vec2F;

// const G: f64 = 6.67430e-11;
const G: f64 = 0.55;

pub struct Simulation {
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
}

impl Simulation {
    pub const fn empty() -> Self {
        Self {
            player: Player::from_nums(&[30.0, 50.0, 120.0]),
            target: Target::from_nums(&[20.0, 330.0, 120.0]),
            planets: Vec::new(),
        }
    }

    /// Replace the contents of the simulation with the newly passed items
    pub fn push(&mut self, player: Player, target: Target, planets: Vec<Planet>) {
        self.player = player;
        self.target = target;
        self.planets = planets;
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

    /// Returns true if the player crashes
    fn gravitate_player(&mut self) -> bool {
        for planet in &self.planets {
            let distance = planet.position - self.player.position;
            let magnitude = distance.x.mul_add(distance.x, distance.y.powi(2));

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

        let distance = vecdistance
            .x
            .mul_add(vecdistance.x, vecdistance.y * vecdistance.y);

        distance < self.target.size.powi(2)
    }
}
