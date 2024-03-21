use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

mod context;
use context::{Context, Vec2F};
mod renderer;
use renderer::{Renderer, GRID_X_SIZE, GRID_Y_SIZE, PIXEL_SCALE};
mod tick;
use tick::GameTime;

use crate::context::AppState;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Orbit simulator",
            GRID_X_SIZE * PIXEL_SCALE,
            GRID_Y_SIZE * PIXEL_SCALE,
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game_tick = GameTime::new();
    let mut context = Context::build("assets/level.obl");
    context.player.acceleration = Vec2F::new(2.5, -0.8);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keymod: Mod::LCTRLMOD,
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,

                Event::MouseMotion { x, y, .. } => {
                    if matches!(context.state, AppState::Aiming) {
                        let distance_to_mouse = Vec2F::new(
                            (x / PIXEL_SCALE as i32) as f64 - context.player.position.x,
                            (y / PIXEL_SCALE as i32) as f64 - context.player.position.y,
                        );

                        let normalised = distance_to_mouse / distance_to_mouse.magnitude();

                        let clamped_distance = distance_to_mouse.magnitude().clamp(30.0, 90.0);
                        let launch_strength = clamped_distance / 30.0;

                        context.player.acceleration = normalised * launch_strength;

                        // display launch strength while aiming
                    }
                }

                _ => (),
            }

            context.event(&event);
        }

        context.tick();

        renderer.draw(&context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
