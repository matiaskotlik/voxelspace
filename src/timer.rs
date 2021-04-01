use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Timer {
    pub elapsed: f32,
    pub duration: f32,
    pub paused: bool,
    pub repeating: bool,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer { duration: duration.as_secs_f32(), ..Default::default() }
    }

    pub fn from_seconds(seconds: f32) -> Self {
        Timer { duration: seconds, ..Default::default() }
    }

    pub fn tick_duration(&mut self, duration: Duration) -> bool {
        self.tick(duration.as_secs_f32())
    }

    pub fn tick(&mut self, delta_seconds: f32) -> bool {
        if self.repeating && self.elapsed >= self.duration {
            self.elapsed -= self.duration;
        }
        if !self.paused {
            self.elapsed += delta_seconds;
        }
        self.elapsed >= self.duration
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer { elapsed: 0.0, duration: 0.0, paused: false, repeating: true }
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn test_repeating() {
        let mut t = Timer::from_seconds(10.0);
        assert_eq!(t.tick(3.0), false);
        assert_eq!(t.tick(7.0), true);
        t.paused = true;
        assert_eq!(t.tick(20.0), false);
        t.paused = false;
        assert_eq!(t.tick(7.0), false);
        assert_eq!(t.tick(3.0), true);
    }
}
