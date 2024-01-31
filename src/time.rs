use std::time::{Instant, Duration};

pub struct TimeStep {
    delta_time: f32,
    last_time: Instant,
    last_sec: Instant,
    frames: u32,
    fps: u32,
}

impl Default for TimeStep {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            last_time: Instant::now(),
            last_sec: Instant::now(),
            frames: 0,
            fps: 0,
        }
    }
}

impl TimeStep {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_millis() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;

        self.frames += 1;
        if current_time.duration_since(self.last_sec) >= Duration::from_secs(1) {
            self.last_sec = current_time;
            self.fps = self.frames;
            self.frames = 0;
        }

        delta
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn frames(&self) -> u32 {
        self.frames
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }
}