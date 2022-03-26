use std::time::Duration;

use bevy::core::Stopwatch;

pub struct VariableTimer {
    stopwatch: Stopwatch,
}

impl VariableTimer {
    pub fn new() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
        }
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    #[inline]
    pub fn set_elapsed(&mut self, time: Duration) {
        self.stopwatch.set_elapsed(time);
    }

    pub fn tick(&mut self, delta: Duration, duration: Duration) -> (u32, f32) {
        self.stopwatch.tick(delta);

        let times_finished = (self.elapsed().as_secs_f32() / duration.as_secs_f32()) as u32;
        self.set_elapsed(self.elapsed() - (duration * times_finished));

        let percent = self.elapsed().as_secs_f32() / duration.as_secs_f32();

        (times_finished, percent)
    }
}
