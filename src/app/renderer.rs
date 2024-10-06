use sdl2::{
    gfx::primitives::DrawRenderer, image::SaveSurface, pixels::Color, rect::Rect,
    render::WindowCanvas, surface::Surface, video::Window,
};

use super::context::{AppState, Context, Simulation, Vec2F};

mod draw_objects;

pub const GRID_X_SIZE: u32 = 400;
pub const GRID_Y_SIZE: u32 = 240;

pub struct Renderer {
    canvas: WindowCanvas,
    pixel_scale: u32,
    pub screenshot_next_frame: bool,
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
            screenshot_next_frame: false,
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

    fn draw_text(&self, x: i16, y: i16, text: &str, colour: Color) -> Result<(), String> {
        for (i, s) in (0..).zip(text.split('\n')) {
            self.canvas.string(x, y + i * 10, s, colour)?;
        }

        Ok(())
    }

    pub fn screenshot(&self) -> Result<(), String> {
        let pixels = self
            .canvas
            .read_pixels(None, self.canvas.default_pixel_format())?;

        let scaled_pixels: Vec<u8> = pixels
            .chunks(4)
            .enumerate()
            .filter_map(|(i, p)| {
                if i % self.pixel_scale as usize > 0 {
                    return None;
                }

                if i / (GRID_X_SIZE * self.pixel_scale) as usize % self.pixel_scale as usize > 0 {
                    return None;
                }

                Some(p.to_vec())
            })
            .flatten()
            .collect();

        let mut screenshot_surface =
            Surface::new(GRID_X_SIZE, GRID_Y_SIZE, self.canvas.default_pixel_format())?
                .into_canvas()?;

        let surface_texture_creator = screenshot_surface.texture_creator();
        let mut screenshot_texture = surface_texture_creator
            .create_texture_target(self.canvas.default_pixel_format(), GRID_X_SIZE, GRID_Y_SIZE)
            .map_err(|e| e.to_string())?;

        screenshot_texture
            .update(
                Rect::new(0, 0, GRID_X_SIZE, GRID_Y_SIZE),
                &scaled_pixels,
                GRID_X_SIZE as usize * 4,
            )
            .map_err(|e| e.to_string())?;

        screenshot_surface.copy(
            &screenshot_texture,
            Rect::new(0, 0, GRID_X_SIZE, GRID_Y_SIZE),
            Rect::new(0, 0, GRID_X_SIZE, GRID_Y_SIZE),
        )?;

        screenshot_surface.into_surface().save("screenshot.png")?;
        println!("saved to screenshot.png");

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

        if matches!(context.state, AppState::Editing)
            && context.level_data.player.velocity != Vec2F::ZERO
        {
            draw_objects::trajectory(&mut self.canvas, context, 2000, 1, Color::RGB(60, 60, 60))?;
        }

        let drawn_level = if matches!(context.state, AppState::Flying | AppState::GameOver(_)) {
            context.simulation.clone()
        } else {
            let mut simulation = Simulation::empty();
            simulation.push(&context.level_data);

            simulation
        };

        draw_objects::planets(&self.canvas, &drawn_level.planets)?;
        draw_objects::player(&self.canvas, &drawn_level.player)?;
        draw_objects::target(&self.canvas, &drawn_level.target)?;
        draw_objects::walls(
            &self.canvas,
            &drawn_level.walls,
            context.state,
            context.edit_selection.show_grab_indicators,
        )?;

        if self.screenshot_next_frame {
            self.screenshot_next_frame = false;
            self.screenshot()?;
        }

        self.draw_hud_text(context)?;

        self.canvas.present();

        Ok(())
    }

    fn draw_hud_text(&self, context: &Context) -> Result<(), String> {
        // Current app state
        self.draw_text(2, 2, &context.state.to_string(), Color::WHITE)?;

        // Helper text
        let helper_text = match (context.show_hints, context.state) {
            (true, AppState::Editing) => String::from("Drag planets and walls with mouse\nChange size by scrolling while holding\nA to spawn a new planet\nW to spawn a wall\nX to delete a selected body"),

            (false, AppState::Aiming) => format!("Launch Strength: {:.2}", context.level_data.player.velocity.magnitude()),
            (true, AppState::Aiming) => format!("Launch Strength: {:.2}\nAim with mouse\n", context.level_data.player.velocity.magnitude()),

            (_, AppState::Flying) => {
                let paused_text = if context.simulation.playing { "" } else { "Paused" };
                format!("Speed x{}\n{paused_text}", context.simulation.speed)
            }
            (true, AppState::GameOver(_)) => String::from("Press Escape to edit\nPress R to restart"),

            (_, _) => String::new(),
        };

        self.draw_text(2, 12, &helper_text, Color::YELLOW)?;

        // Current level path
        let display_path = context
            .level_path
            .split('/')
            .last()
            .unwrap_or(&context.level_path);
        self.draw_text(2, 230, &format!("Editing {display_path}"), Color::WHITE)
    }
}
