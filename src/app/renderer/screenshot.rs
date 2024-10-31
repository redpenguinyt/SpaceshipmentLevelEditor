use sdl2::{image::SaveSurface, rect::Rect, render::Canvas, surface::Surface, video::Window};

use crate::app::renderer::{GRID_X_SIZE, GRID_Y_SIZE};

pub fn screenshot(canvas: &Canvas<Window>, pixel_scale: u32, level_path: &str) -> Result<(), String> {
    let pixels = canvas.read_pixels(None, canvas.default_pixel_format())?;

    let scaled_pixels: Vec<u8> = pixels
        .chunks(4)
        .enumerate()
        .filter_map(|(i, p)| {
            if i % pixel_scale as usize > 0 {
                return None;
            }

            if i / (GRID_X_SIZE * pixel_scale) as usize % pixel_scale as usize > 0 {
                return None;
            }

            Some(p.to_vec())
        })
        .flatten()
        .collect();

    let mut screenshot_surface =
        Surface::new(GRID_X_SIZE, GRID_Y_SIZE, canvas.default_pixel_format())?.into_canvas()?;

    let surface_texture_creator = screenshot_surface.texture_creator();
    let mut screenshot_texture = surface_texture_creator
        .create_texture_target(canvas.default_pixel_format(), GRID_X_SIZE, GRID_Y_SIZE)
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

    let mut screenshot_path: String = level_path.strip_suffix(".obl").unwrap_or(level_path).into();
    screenshot_path.push_str("-screenshot.png");
    screenshot_surface.into_surface().save(&screenshot_path)?;
    println!("saved screenshot to {screenshot_path}");

    Ok(())
}
