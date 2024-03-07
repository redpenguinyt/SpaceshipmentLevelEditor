use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

mod context;
use context::{Context, Vec2F};
mod renderer;
use renderer::{Renderer, GRID_X_SIZE, GRID_Y_SIZE, PIXEL_SCALE};
mod tick;
use tick::GameTime;

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
    let mut context = Context::build("../Orbits/Source/levels/level1.obl");
    context.player.acceleration = Vec2F::new(2.5, -1.2);

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
                    keycode: Some(Keycode::P),
                    ..
                } => game_tick.state = !game_tick.state,

                _ => (),
            }
        }

        if game_tick.next_frame() {
            context.tick();
        };

        renderer.draw(&context, &game_tick)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
