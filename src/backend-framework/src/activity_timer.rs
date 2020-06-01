use std::time::{Instant, Duration};

/// Tracks time since last activity, for keep-alive purposes.
pub struct ActivityTracker {
    time_of_last_activity: Instant,
}

impl ActivityTracker {
    pub fn new() -> Self {
        ActivityTracker {
            time_of_last_activity: Instant::now()
        }
    }

    pub fn ping(&mut self) {
        self.time_of_last_activity = Instant::now();
    }

    pub fn is_expired(&self, expiration_duration: Duration) -> bool {
        expiration_duration <= Instant::now()
            .saturating_duration_since(self.time_of_last_activity)
    }
}

#[cfg(test)]
mod tests {
    use crate::activity_timer::ActivityTracker;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test() {
        let mut t = ActivityTracker::new();
        assert!(!t.is_expired(Duration::from_millis(50)));
        thread::sleep(Duration::from_millis(50));
        assert!(t.is_expired(Duration::from_millis(50)));

        t.ping();
        assert!(!t.is_expired(Duration::from_millis(50)));
    }
}