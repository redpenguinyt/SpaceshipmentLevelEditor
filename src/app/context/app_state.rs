use std::fmt::Display;

use super::SimulationEvent;

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Editing,
    Aiming,
    Flying,
    GameOver(SimulationEvent),
}

impl AppState {
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Editing => Self::Aiming,
            _ => Self::Editing,
        }
    }
}

impl Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_text = match *self {
            Self::Editing => "Edit Mode",
            Self::Aiming => "Aim Mode",
            Self::Flying => "Simulating...",
            Self::GameOver(SimulationEvent::Won) => "Target reached!",
            Self::GameOver(SimulationEvent::Crashed) => "Probe crashed!",
        };

        write!(f, "{display_text}")?;

        Ok(())
    }
}
