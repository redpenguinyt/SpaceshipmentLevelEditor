mod fonts;

use std::f64::consts::PI;

use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Point,
    render::WindowCanvas,
    video::Window,
};

use crate::context::{AppState, Context, Planet, Player, Target};

use self::fonts::FontHandler;

pub const GRID_X_SIZE: u32 = 400;
pub const GRID_Y_SIZE: u32 = 240;
pub const PIXEL_SCALE: u32 = 4;

pub struct Renderer {
    canvas: WindowCanvas,
    font: FontHandler,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self {
            canvas,
            font: FontHandler::new("assets/arcade.ttf")?,
        })
    }

    // fn plot(&mut self, point: Point, color: Color) -> Result<(), String> {
    //     self.canvas.set_draw_color(color);

    //     self.canvas.fill_rect(Rect::new(
    //         point.x * PIXEL_SCALE as i32,
    //         point.y * PIXEL_SCALE as i32,
    //         PIXEL_SCALE,
    //         PIXEL_SCALE,
    //     ))?;

    //     Ok(())
    // }

    fn draw_background(&mut self, state: AppState) -> Result<(), String> {
        let colour = if matches!(state, AppState::Editing) {
            Color::RGB(10, 10, 10)
        } else {
            Color::RGB(0, 0, 0)
        };

        self.canvas.set_draw_color(colour);
        self.canvas.clear();

        self.font.draw_text(
            &mut self.canvas,
            &format!("{state:?}"),
            Point::new(10, 10),
            32,
        )
    }

    fn draw_planets(&mut self, planets: &[Planet]) -> Result<(), String> {
        for planet in planets {
            self.canvas.filled_circle(
                (planet.position.x.round() * PIXEL_SCALE as f64) as i16,
                (planet.position.y.round() * PIXEL_SCALE as f64) as i16,
                ((planet.mass / 12.0).round() * PIXEL_SCALE as f64) as i16,
                Color::GREY,
            )?;
        }

        Ok(())
    }

    fn draw_player(&mut self, player: &Player) -> Result<(), String> {
        let angle = player.acceleration.y.atan2(player.acceleration.x);

        let pos_x = (player.position.x.round() * PIXEL_SCALE as f64) as i16;
        let pos_y = (player.position.y.round() * PIXEL_SCALE as f64) as i16;

        self.canvas.filled_trigon(
            pos_x + (30.0 * angle.cos()).round() as i16,
            pos_y + (30.0 * angle.sin()).round() as i16,
            pos_x + (30.0 * PI.mul_add(-0.8, angle).cos()).round() as i16,
            pos_y + (30.0 * PI.mul_add(-0.8, angle).sin()).round() as i16,
            pos_x + (30.0 * PI.mul_add(0.8, angle).cos()).round() as i16,
            pos_y + (30.0 * PI.mul_add(0.8, angle).sin()).round() as i16,
            Color::WHITE,
        )
    }

    fn draw_target(&mut self, target: &Target) -> Result<(), String> {
        self.canvas.circle(
            (target.position.x.round() * PIXEL_SCALE as f64) as i16,
            (target.position.y.round() * PIXEL_SCALE as f64) as i16,
            (target.size.round() * PIXEL_SCALE as f64) as i16,
            Color::GREY,
        )
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context.state)?;

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
