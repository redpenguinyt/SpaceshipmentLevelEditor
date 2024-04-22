use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas, video::Window};

use super::context::{AppState, Context, Vec2F};

mod draw_objects;

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
            .map_err(|e| e.to_string())?;

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
        for (i, s) in (0..).zip(text.split('\n')) {
            self.canvas.string(x, y + i * 10, s, colour)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        let image_background_path = context
            .show_background_image
            .then(|| context.level_path.replace("obl", "png"));

        draw_objects::background(&mut self.canvas, context.state, image_background_path)?;

        if matches!(context.state, AppState::Aiming) {
            draw_objects::trajectory(&mut self.canvas, context, 2000, 1, Color::GREY)?;
            draw_objects::trajectory(&mut self.canvas, context, 15, 4, Color::WHITE)?;
        }

        if matches!(context.state, AppState::Editing) && context.player.velocity != Vec2F::ZERO {
            draw_objects::trajectory(&mut self.canvas, context, 2000, 1, Color::RGB(60, 60, 60))?;
        }

        if matches!(context.state, AppState::Flying | AppState::GameOver(_)) {
            draw_objects::planets(&self.canvas, &context.simulation.planets)?;
            draw_objects::player(&self.canvas, &context.simulation.player)?;
            draw_objects::target(&self.canvas, &context.simulation.target)?;
            draw_objects::walls(
                &self.canvas,
                &context.simulation.walls,
                context.state,
                context.edit_selection.show_grab_indicators,
            )?;
        } else {
            draw_objects::planets(&self.canvas, &context.planets)?;
            draw_objects::player(&self.canvas, &context.player)?;
            draw_objects::target(&self.canvas, &context.target)?;
            draw_objects::walls(
                &self.canvas,
                &context.walls,
                context.state,
                context.edit_selection.show_grab_indicators,
            )?;
        }

        // Current app state
        self.draw_text(2, 2, &context.state.to_string(), Color::WHITE)?;

        // Helper text
        let helper_text = match (context.show_hints, context.state) {
            (true, AppState::Editing) => String::from("Drag planets with mouse\nChange size by scrolling while holding\nA to spawn a new planet"),
            (true, AppState::Aiming) => format!("Launch Strength: {:.2}\nAim with mouse\nBring mouse closer to player to lower launch strength", context.player.velocity.magnitude()),
            (_, AppState::Flying) => format!("Speed x{}", context.simulation.speed),
            (true, AppState::GameOver(_)) => String::from("Press R to restart"),

            (false, AppState::Aiming) => format!("Launch Strength: {:.2}", context.player.velocity.magnitude()),

            (_, _) => String::new(),
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
