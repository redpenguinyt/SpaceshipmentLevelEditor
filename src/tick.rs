use std::{
    ops::Not,
    thread,
    time::{Duration, Instant},
};

const FPS: u32 = 50;
const TICK_RATE: u32 = 1;

#[derive(Debug, Clone, Copy)]
pub enum Tick {
    Playing,
    Paused,
}

impl Not for Tick {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Paused => Self::Playing,
            Self::Playing => Self::Paused,
        }
    }
}

pub struct GameTime {
    frame_started: Instant,
    pub tick: u32,
    pub tick_rate: u32,
    pub state: Tick,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            frame_started: Instant::now(),
            tick: 0,
            tick_rate: TICK_RATE,
            state: Tick::Playing,
        }
    }

    pub fn next_frame(&mut self) -> bool {
        self.tick += 1;

        if self.tick >= self.tick_rate {
            self.tick = 0;

            match self.state {
                Tick::Playing => return true,
                Tick::Paused => (),
            }
        };

        false
    }

    pub fn sleep_frame(&mut self) {
        let elapsed = self.frame_started.elapsed();

        if elapsed < Duration::from_secs(1) / FPS {
            thread::sleep(Duration::from_secs(1) / FPS - elapsed);
        }

        self.frame_started = Instant::now();
    }
}
