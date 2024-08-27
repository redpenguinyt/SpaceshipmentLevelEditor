use rfd::FileDialog;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

mod context;
mod renderer;
mod tick;
use context::{get_last_file_in_dir, Context, SaveMethod};
use renderer::{Renderer, GRID_X_SIZE, GRID_Y_SIZE};
use tick::GameTime;

const INITIAL_PIXEL_SCALE: u32 = 3;

fn global_keybinds(
    event: &Event,
    renderer: &mut Renderer,
    context: &mut Context,
) -> Result<(), String> {
    match event {
        Event::KeyDown {
            keymod,
            keycode: Some(keycode),
            ..
        } if keymod.contains(Mod::LALTMOD) && (49..=53).contains(&(*keycode as i32)) => {
            // Num1 to Num5
            renderer.change_scale(*keycode as u32 - 48)?;
        }

        Event::KeyDown {
            keymod,
            keycode: Some(Keycode::S),
            ..
        } if keymod.contains(Mod::LCTRLMOD | Mod::LALTMOD) => {
            context.save(SaveMethod::Incremental)?;
            println!("Saved incrementally to {}", context.level_path);
        }

        Event::KeyDown {
            keymod,
            keycode: Some(Keycode::S),
            ..
        } if keymod.contains(Mod::LCTRLMOD | Mod::LSHIFTMOD) => {
            let level = FileDialog::new()
                .set_title("Save Level as")
                .add_filter("Orbit Level", &["obl"])
                .set_directory("./levels/")
                .set_file_name(&context.level_path)
                .set_can_create_directories(true)
                .save_file();

            if let Some(l) = level {
                let Some(path) = l.to_str() else {
                    return Err(String::from("Path is not valid unicode"));
                };

                context.save(SaveMethod::As(String::from(path)))?;
                println!("Saved as {}", context.level_path);
            }
        }

        Event::KeyDown {
            keymod,
            keycode: Some(Keycode::S),
            ..
        } if keymod.contains(Mod::LCTRLMOD) => {
            context.save(SaveMethod::ToCurrentFile)?;
            println!("Saved to {}", context.level_path);
        }

        Event::KeyDown {
            keymod,
            keycode: Some(Keycode::O),
            ..
        } if keymod.contains(Mod::LCTRLMOD) => {
            println!("Opening file select dialog");
            let level = FileDialog::new()
                .set_title("Select Level")
                .add_filter("Orbit Level", &["obl"])
                .set_directory("./levels/")
                .set_file_name(&context.level_path)
                .set_can_create_directories(true)
                .pick_file();

            if let Some(l) = level {
                let Some(path) = l.to_str() else {
                    return Err(String::from("Path is not valid unicode"));
                };

                *context = Context::build(path)?;
            }
        }

        Event::KeyDown {
            keycode: Some(Keycode::F2),
            ..
        } => {
            renderer.screenshot_next_frame = true;
        }

        _ => (),
    }

    Ok(())
}

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

    let level_path = get_last_file_in_dir("levels/")?;

    let mut game_tick = GameTime::new();
    let mut context = Context::build(&level_path)?;

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

            global_keybinds(&event, &mut renderer, &mut context)?;

            context.event(&event);
        }

        context.tick();

        renderer.draw(&context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
