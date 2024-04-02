use std::{
    thread,
    time::{Duration, Instant},
};

const FPS: u32 = 50;

pub struct GameTime {
    frame_started: Instant,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            frame_started: Instant::now(),
        }
    }

    pub fn sleep_frame(&mut self) {
        let elapsed = self.frame_started.elapsed();

        if elapsed < Duration::from_secs(1) / FPS {
            thread::sleep(Duration::from_secs(1) / FPS - elapsed);
        }

        self.frame_started = Instant::now();
    }
}
