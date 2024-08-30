use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use sdl2::{keyboard::Keycode, rect::Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2F {
    pub x: f64,
    pub y: f64,
}

impl Vec2F {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const fn from_mouse_pos(x: i32, y: i32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y)
    }
}

impl Add<Self> for Vec2F {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2F {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Self> for Vec2F {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2F {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for Vec2F {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2F {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Display for Vec2F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl From<Vec2F> for Point {
    fn from(value: Vec2F) -> Self {
        Self::new(value.x.round() as i32, value.y.round() as i32)
    }
}

impl TryFrom<&Keycode> for Vec2F {
    type Error = String;

    fn try_from(value: &Keycode) -> Result<Self, Self::Error> {
        match value {
            Keycode::Up => Ok(Self::new(0.0, -1.0)),
            Keycode::Down => Ok(Self::new(0.0, 1.0)),
            Keycode::Left => Ok(Self::new(-1.0, 0.0)),
            Keycode::Right => Ok(Self::new(1.0, 0.0)),

            _ => Err(String::from("Value is not an arrow key")),
        }
    }
}
