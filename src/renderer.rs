use std::f64::consts::PI;

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas, video::Window};

use crate::context::{AppState, Context, Planet, Player, Simulation, SimulationEvent, Target};

pub const GRID_X_SIZE: u32 = 400;
pub const GRID_Y_SIZE: u32 = 240;
pub const PIXEL_SCALE: u32 = 3;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self, String> {
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas
            .set_logical_size(GRID_X_SIZE, GRID_Y_SIZE)
            .expect("well that failed");

        Ok(Self { canvas })
    }

    fn draw_text(&mut self, x: i16, y: i16, text: &str, colour: Color) -> Result<(), String> {
        for (i, c) in (0i16..).zip(text.chars()) {
            self.canvas.character(x + 7 * i, y, c, colour)?;
        }

        Ok(())
    }

    fn draw_background(&mut self, state: AppState) -> Result<(), String> {
        let colour = if matches!(state, AppState::Editing) {
            Color::RGB(10, 10, 10)
        } else {
            Color::RGB(0, 0, 0)
        };

        self.canvas.set_draw_color(colour);
        self.canvas.clear();

        self.draw_text(2, 2, &format!("{state:?}"), Color::WHITE)?;

        Ok(())
    }

    fn draw_planets(&mut self, planets: &[Planet]) -> Result<(), String> {
        for planet in planets {
            self.canvas.filled_circle(
                (planet.pos.x.round()) as i16,
                (planet.pos.y.round()) as i16,
                ((planet.mass / 12.0).round()) as i16,
                Color::GREY,
            )?;
        }

        Ok(())
    }

    fn draw_player(&mut self, player: &Player) -> Result<(), String> {
        let angle = player.acceleration.y.atan2(player.acceleration.x);

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
        );

        let mut last_pos = simulation.player.pos;

        let mut has_crashed = false;
        for _ in 0..count {
            if has_crashed {
                break;
            }

            for _ in 0..spacing {
                if matches!(simulation.tick(), Some(SimulationEvent::Crashed)) {
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
        self.draw_background(context.state)?;

        if matches!(context.state, AppState::Aiming) {
            self.draw_trajectory(context, 2000, 1, Color::GREY)?;
            self.draw_trajectory(context, 15, 4, Color::WHITE)?;
        }

        if matches!(context.state, AppState::Flying) {
            self.draw_planets(&context.simulation.planets)?;
            self.draw_player(&context.simulation.player)?;
            self.draw_target(&context.simulation.target)?;
        } else {
            self.draw_planets(&context.planets)?;
            self.draw_player(&context.player)?;
            self.draw_target(&context.target)?;
        }

        self.canvas.present();

        Ok(())
    }
}
