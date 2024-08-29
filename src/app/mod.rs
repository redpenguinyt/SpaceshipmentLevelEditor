use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

mod context;
mod renderer;
mod tick;
use context::Context;
use renderer::{Renderer, GRID_X_SIZE, GRID_Y_SIZE};
use tick::GameTime;

const INITIAL_PIXEL_SCALE: u32 = 3;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Orbit Game Editor",
            GRID_X_SIZE * INITIAL_PIXEL_SCALE,
            GRID_Y_SIZE * INITIAL_PIXEL_SCALE,
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window, INITIAL_PIXEL_SCALE)?;

    let mut event_pump = sdl_context.event_pump()?;

    // let level_path = get_last_file_in_dir("levels/")?;

    let mut game_tick = GameTime::new();
    let mut context = Context::new();

    let mut args = std::env::args().skip(1);
    if let Some(path) = args.next() {
        context.load(&path)?;
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            if matches!(
                event,
                Event::Quit { .. }
                    | Event::KeyDown {
                        keymod: Mod::LCTRLMOD,
                        keycode: Some(Keycode::Q),
                        ..
                    }
            ) {
                break 'running;
            }

            let global_keybind_pressed = context.global_keybinds(&event, &mut renderer)?;

            if !global_keybind_pressed {
                context.event(&event);
            }
        }

        context.tick();

        renderer.draw(&context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
