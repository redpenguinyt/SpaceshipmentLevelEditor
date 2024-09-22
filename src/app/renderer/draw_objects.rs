use sdl2::{
    gfx::primitives::DrawRenderer, image::LoadTexture, pixels::Color, rect::Rect,
    render::WindowCanvas,
};
use std::f64::consts::PI;

use super::{GRID_X_SIZE, GRID_Y_SIZE};
use crate::app::context::{AppState, Context, Planet, Player, Simulation, Target, Wall};

pub fn background(
    canvas: &mut WindowCanvas,
    state: AppState,
    background_path: Option<String>,
) -> Result<(), String> {
    let colour = if matches!(state, AppState::Editing) {
        Color::RGB(10, 10, 10)
    } else {
        Color::RGB(0, 0, 0)
    };

    canvas.set_draw_color(colour);
    canvas.clear();

    if let Some(bg_path) = background_path {
        let texture_creator = canvas.texture_creator();
        let load_texture = texture_creator.load_texture(&bg_path);

        match load_texture {
            Ok(texture) => {
                canvas.copy(&texture, Rect::new(0, 0, GRID_X_SIZE, GRID_Y_SIZE), None)?;
            }
            Err(e) => {
                if !e.contains("Couldn't open ") {
                    println!("Background image not loaded successfully: {e}");
                }
            }
        }
    }

    Ok(())
}

pub fn planets(canvas: &WindowCanvas, planets: &[Planet]) -> Result<(), String> {
    for planet in planets {
        let pos = ((planet.pos.x.round()) as i16, (planet.pos.y.round()) as i16);
        let radius = ((planet.mass.abs() / 12.0).round()) as i16;

        if planet.mass.is_sign_positive() {
            canvas.filled_circle(pos.0, pos.1, radius, Color::GREY)?;
        } else {
            canvas.circle(pos.0, pos.1, radius, Color::GREY)?;
        };
    }

    Ok(())
}

pub fn player(canvas: &WindowCanvas, player: &Player) -> Result<(), String> {
    let angle = player.velocity.angle();

    let pos_x = (player.pos.x.round()) as i16;
    let pos_y = (player.pos.y.round()) as i16;

    canvas.filled_trigon(
        pos_x + (8.0 * angle.cos()).round() as i16,
        pos_y + (8.0 * angle.sin()).round() as i16,
        pos_x + (8.0 * PI.mul_add(-0.8, angle).cos()).round() as i16,
        pos_y + (8.0 * PI.mul_add(-0.8, angle).sin()).round() as i16,
        pos_x + (8.0 * PI.mul_add(0.8, angle).cos()).round() as i16,
        pos_y + (8.0 * PI.mul_add(0.8, angle).sin()).round() as i16,
        Color::WHITE,
    )
}

pub fn target(canvas: &WindowCanvas, target: &Target) -> Result<(), String> {
    canvas.circle(
        (target.pos.x.round()) as i16,
        (target.pos.y.round()) as i16,
        (target.size.round()) as i16,
        Color::GREEN,
    )
}

pub fn walls(
    canvas: &WindowCanvas,
    walls: &[Wall],
    state: AppState,
    show_grab_indicators: bool,
) -> Result<(), String> {
    for wall in walls {
        canvas.thick_line(
            wall.pos1.x.round() as i16,
            wall.pos1.y.round() as i16,
            wall.pos2.x.round() as i16,
            wall.pos2.y.round() as i16,
            2,
            Color::RGB(200, 200, 200),
        )?;

        if matches!(state, AppState::Editing) && show_grab_indicators {
            canvas.circle(
                wall.pos1.x.round() as i16,
                wall.pos1.y.round() as i16,
                7,
                Color::RGB(200, 200, 200),
            )?;
            canvas.circle(
                wall.pos2.x.round() as i16,
                wall.pos2.y.round() as i16,
                7,
                Color::RGB(200, 200, 200),
            )?;
        }
    }

    Ok(())
}

pub fn trajectory(
    canvas: &mut WindowCanvas,
    context: &Context,
    count: i32,
    spacing: i32,
    colour: Color,
) -> Result<(), String> {
    let mut simulation = Simulation::empty();
    simulation.push(&context.level_data);

    let mut last_pos = simulation.player.pos;

    let mut has_crashed = false;
    for _ in 0..count {
        if has_crashed {
            break;
        }

        for _ in 0..spacing {
            if simulation.tick().is_some() {
                has_crashed = true;
                break;
            }
        }

        canvas.set_draw_color(colour);
        canvas.draw_line(last_pos, simulation.player.pos)?;

        last_pos = simulation.player.pos;
    }

    Ok(())
}
