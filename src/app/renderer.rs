use std::f64::consts::PI;

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas, video::Window};

use super::context::{AppState, Context, Planet, Player, Simulation, Target, Wall};

pub const GRID_X_SIZE: u32 = 400;
pub const GRID_Y_SIZE: u32 = 240;

pub struct Renderer {
    canvas: WindowCanvas,
    pixel_scale: u32,
}

impl Renderer {
    pub fn new(window: Window, pixel_scale: u32) -> Result<Self, String> {
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas
            .window_mut()
            .set_size(GRID_X_SIZE * pixel_scale, GRID_Y_SIZE * pixel_scale)
            .map_err(|e| e.to_string())?;

        canvas
            .set_logical_size(GRID_X_SIZE, GRID_Y_SIZE)
            .expect("well that failed");

        Ok(Self {
            canvas,
            pixel_scale,
        })
    }

    pub fn change_scale(&mut self, scale: u32) -> Result<(), String> {
        self.pixel_scale = scale;

        self.canvas
            .window_mut()
            .set_size(
                GRID_X_SIZE * self.pixel_scale,
                GRID_Y_SIZE * self.pixel_scale,
            )
            .map_err(|e| e.to_string())
    }

    fn draw_text(&mut self, x: i16, y: i16, text: &str, colour: Color) -> Result<(), String> {
        let mut i = 0;
        let mut line = 0;

        for c in text.chars() {
            if c == '\n' {
                i = 0;
                line += 1;
                continue;
            }
            self.canvas.character(x + 7 * i, y + line * 10, c, colour)?;

            i += 1;
        }

        Ok(())
    }

    fn draw_background(&mut self, state: AppState) {
        let colour = if matches!(state, AppState::Editing) {
            Color::RGB(10, 10, 10)
        } else {
            Color::RGB(0, 0, 0)
        };

        self.canvas.set_draw_color(colour);
        self.canvas.clear();
    }

    fn draw_planets(&mut self, planets: &[Planet]) -> Result<(), String> {
        for planet in planets {
            let pos = ((planet.pos.x.round()) as i16, (planet.pos.y.round()) as i16);
            let radius = ((planet.mass.abs() / 12.0).round()) as i16;

            if planet.mass.is_sign_positive() {
                self.canvas
                    .filled_circle(pos.0, pos.1, radius, Color::GREY)?;
            } else {
                self.canvas.circle(pos.0, pos.1, radius, Color::GREY)?;
            };
        }

        Ok(())
    }

    fn draw_player(&mut self, player: &Player) -> Result<(), String> {
        let angle = player.velocity.y.atan2(player.velocity.x);

        let pos_x = (player.pos.x.round()) as i16;
        let pos_y = (player.pos.y.round()) as i16;

        self.canvas.filled_trigon(
            pos_x + (8.0 * angle.cos()).round() as i16,
            pos_y + (8.0 * angle.sin()).round() as i16,
            pos_x + (8.0 * PI.mul_add(-0.8, angle).cos()).round() as i16,
            pos_y + (8.0 * PI.mul_add(-0.8, angle).sin()).round() as i16,
            pos_x + (8.0 * PI.mul_add(0.8, angle).cos()).round() as i16,
            pos_y + (8.0 * PI.mul_add(0.8, angle).sin()).round() as i16,
            Color::WHITE,
        )
    }

    fn draw_target(&mut self, target: &Target) -> Result<(), String> {
        self.canvas.circle(
            (target.pos.x.round()) as i16,
            (target.pos.y.round()) as i16,
            (target.size.round()) as i16,
            Color::GREEN,
        )
    }

    fn draw_walls(&mut self, walls: &[Wall], state: AppState) -> Result<(), String> {
        for wall in walls {
            self.canvas.thick_line(
                wall.pos1.x.round() as i16,
                wall.pos1.y.round() as i16,
                wall.pos2.x.round() as i16,
                wall.pos2.y.round() as i16,
                1,
                Color::RGB(200, 200, 200),
            )?;

            if matches!(state, AppState::Editing) {
                self.canvas.circle(
                    wall.pos1.x.round() as i16,
                    wall.pos1.y.round() as i16,
                    7,
                    Color::RGB(200, 200, 200),
                )?;
                self.canvas.circle(
                    wall.pos2.x.round() as i16,
                    wall.pos2.y.round() as i16,
                    7,
                    Color::RGB(200, 200, 200),
                )?;
            }
        }

        Ok(())
    }

    fn draw_trajectory(
        &mut self,
        context: &Context,
        count: i32,
        spacing: i32,
        colour: Color,
    ) -> Result<(), String> {
        let mut simulation = Simulation::empty();
        simulation.push(
            context.player.clone(),
            context.target.clone(),
            context.planets.clone(),
            context.walls.clone(),
        );

        let mut last_pos = simulation.player.pos;

        let mut has_crashed = false;
        for _ in 0..count {
            if has_crashed {
                break;
            }

            for _ in 0..spacing {
                if simulation.tick().is_some() {
                    has_crashed = true;
                    break;
                }
            }

            self.canvas.set_draw_color(colour);
            self.canvas.draw_line(last_pos, simulation.player.pos)?;

            last_pos = simulation.player.pos;
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context.state);

        if matches!(context.state, AppState::Aiming) {
            self.draw_trajectory(context, 2000, 1, Color::GREY)?;
            self.draw_trajectory(context, 15, 4, Color::WHITE)?;
        }

        if matches!(context.state, AppState::Flying | AppState::GameOver(_)) {
            self.draw_planets(&context.simulation.planets)?;
            self.draw_player(&context.simulation.player)?;
            self.draw_target(&context.simulation.target)?;
            self.draw_walls(&context.simulation.walls, context.state)?;
        } else {
            self.draw_planets(&context.planets)?;
            self.draw_player(&context.player)?;
            self.draw_target(&context.target)?;
            self.draw_walls(&context.walls, context.state)?;
        }

        // Current app state
        self.draw_text(2, 2, &context.state.to_string(), Color::WHITE)?;

        // Helper text
        let helper_text = match context.state {
            AppState::Editing => String::from("Drag planets with mouse\nChange size by scrolling while holding\nA to spawn a new planet"),
            AppState::Aiming => format!("Launch Strength: {:.2}\nAim with mouse\nBring mouse closer to player to lower launch strength",
            context.player.velocity.magnitude()),
            AppState::Flying => format!("Speed x{}", context.simulation_speed),
            AppState::GameOver(_) => String::from("Press R to restart"),
        };
        self.draw_text(2, 12, &helper_text, Color::YELLOW)?;

        // Current level path
        let mut display_path = context.level_path.clone();
        if display_path.len() > 47 {
            let Some(split_pos) = display_path.char_indices().nth_back(40) else {
                return Err(String::from("Failed to display path"));
            };

            display_path = format!("...{}", &display_path[split_pos.0..].trim());
        }
        self.draw_text(2, 230, &format!("Editing: {display_path}"), Color::WHITE)?;

        self.canvas.present();

        Ok(())
    }
}
