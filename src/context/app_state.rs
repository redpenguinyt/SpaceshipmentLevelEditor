#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Editing,
    Aiming,
    Flying,
}

impl AppState {
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Editing => Self::Aiming,
            _ => Self::Editing,
        }
    }
}
