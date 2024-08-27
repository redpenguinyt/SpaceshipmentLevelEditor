use rfd::FileDialog;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

use crate::app::Renderer;
use super::SaveMethod;

impl super::Context {
    pub fn global_keybinds(&mut self, event: &Event, renderer: &mut Renderer) -> Result<(), String> {
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
            } if keymod.contains(Mod::LCTRLMOD | Mod::LSHIFTMOD)
                || self.level_path == "new level" =>
            {
                if self.level_path == "new_level" {
                    self.level_path = "new_level.obl".into();
                }
                let level = FileDialog::new()
                    .set_title("Save Level as")
                    .add_filter("Spaceshipment Level", &["obl"])
                    .set_directory("./levels/")
                    .set_file_name(&self.level_path)
                    .set_can_create_directories(true)
                    .save_file();

                if let Some(l) = level {
                    let Some(path) = l.to_str() else {
                        return Err(String::from("Path is not valid unicode"));
                    };

                    self.save(SaveMethod::As(String::from(path)))?;
                    println!("Saved as {}", self.level_path);
                }
            }

            Event::KeyDown {
                keymod,
                keycode: Some(Keycode::S),
                ..
            } if keymod.contains(Mod::LCTRLMOD | Mod::LALTMOD) => {
                self.save(SaveMethod::Incremental)?;
                println!("Saved incrementally to {}", self.level_path);
            }

            Event::KeyDown {
                keymod,
                keycode: Some(Keycode::S),
                ..
            } if keymod.contains(Mod::LCTRLMOD) => {
                self.save(SaveMethod::ToCurrentFile)?;
                println!("Saved to {}", self.level_path);
            }

            Event::KeyDown {
                keymod,
                keycode: Some(Keycode::N),
                ..
            } if keymod.contains(Mod::LCTRLMOD) => {
                let show_hints = self.show_hints;
                let show_background_image = self.show_background_image;
                *self = Self::new();
                self.show_hints = show_hints;
                self.show_background_image = show_background_image;
                println!("Opened new level");
            }

            Event::KeyDown {
                keymod,
                keycode: Some(Keycode::O),
                ..
            } if keymod.contains(Mod::LCTRLMOD) => {
                println!("Opening file select dialog");
                let level = FileDialog::new()
                    .set_title("Select Level")
                    .add_filter("Spaceshipment Level", &["obl"])
                    .set_directory("./levels/")
                    .set_file_name(&self.level_path)
                    .set_can_create_directories(true)
                    .pick_file();

                if let Some(l) = level {
                    let Some(path) = l.to_str() else {
                        return Err(String::from("Path is not valid unicode"));
                    };

                    self.load(path)?;
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
}
