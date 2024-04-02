use dialog::DialogBox;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

mod context;
use context::{get_last_file_in_dir, Context, Vec2F};
mod renderer;
use renderer::{Renderer, GRID_X_SIZE, GRID_Y_SIZE};
mod tick;
use tick::GameTime;

const INITIAL_PIXEL_SCALE: u32 = 3;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Orbit simulator",
            GRID_X_SIZE * INITIAL_PIXEL_SCALE,
            GRID_Y_SIZE * INITIAL_PIXEL_SCALE,
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window, INITIAL_PIXEL_SCALE)?;

    let mut event_pump = sdl_context.event_pump()?;

    let level_path = get_last_file_in_dir("levels/")?;

    let mut game_tick = GameTime::new();
    let mut context = Context::build(&level_path);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keymod: Mod::LCTRLMOD,
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keymod: Mod::LALTMOD,
                    keycode: Some(keycode),
                    ..
                } if (49..=53).contains(&(keycode as i32)) => {
                    // Num1 to Num5
                    renderer.change_scale(keycode as u32 - 48)?;
                }

                Event::KeyDown {
                    keymod: Mod::LCTRLMOD,
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    context.save(false)?;
                    println!("Saved to {}", context.level_path);
                }

                Event::KeyDown {
                    keymod,
                    keycode: Some(Keycode::S),
                    ..
                } if keymod.contains(Mod::LCTRLMOD | Mod::LSHIFTMOD) => {
                    context.save(true)?;
                    println!("Saved as {}", context.level_path);
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
