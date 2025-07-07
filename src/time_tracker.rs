use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TimeTracker {
    start_time: Instant,
    total_tiles: usize,
    completed_tiles: usize,
}

impl TimeTracker {
    pub fn new(total_tiles: usize) -> Self {
        Self {
            start_time: Instant::now(),
            total_tiles,
            completed_tiles: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.completed_tiles = 0;
    }

    pub fn tick(&mut self) {
        self.completed_tiles += 1;
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn eta(&self) -> Option<Duration> {
        if self.completed_tiles == 0 {
            return None;
        }

        let elapsed = self.elapsed();
        let average_time_per_tile = elapsed.as_secs_f64() / self.completed_tiles as f64;
        let remaining_tiles = self.total_tiles - self.completed_tiles;

        if remaining_tiles == 0 {
            return Some(Duration::from_secs(0));
        }

        let eta_seconds = average_time_per_tile * remaining_tiles as f64;
        Some(Duration::from_secs_f64(eta_seconds))
    }

    pub fn progress(&self) -> f64 {
        if self.total_tiles == 0 {
            return 1.0;
        }
        self.completed_tiles as f64 / self.total_tiles as f64
    }

    pub fn completed_tiles(&self) -> usize {
        self.completed_tiles
    }

    pub fn total_tiles(&self) -> usize {
        self.total_tiles
    }

    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let millis = duration.subsec_millis();

        if hours > 0 {
            format!("{hours}h {minutes:02}m {seconds:02}s")
        } else if minutes > 0 {
            format!("{minutes}m {seconds:02}s")
        } else {
            format!("{seconds}.{millis:03}s")
        }
    }

    pub fn format_elapsed(&self) -> String {
        Self::format_duration(self.elapsed())
    }

    pub fn format_eta(&self) -> String {
        match self.eta() {
            Some(eta) => format!("ETA: {}", Self::format_duration(eta)),
            None => "ETA: --".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Completed: {}/{} tiles in {} ({})",
            self.completed_tiles,
            self.total_tiles,
            self.format_elapsed(),
            self.format_eta()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_time_tracker_creation() {
        let tracker = TimeTracker::new(100);
        assert_eq!(tracker.total_tiles(), 100);
        assert_eq!(tracker.completed_tiles(), 0);
        assert_eq!(tracker.progress(), 0.0);
    }

    #[test]
    fn test_time_tracker_tick() {
        let mut tracker = TimeTracker::new(10);
        tracker.tick();
        assert_eq!(tracker.completed_tiles(), 1);
        assert_eq!(tracker.progress(), 0.1);

        tracker.tick();
        assert_eq!(tracker.completed_tiles(), 2);
        assert_eq!(tracker.progress(), 0.2);
    }

    #[test]
    fn test_time_tracker_elapsed() {
        let mut tracker = TimeTracker::new(5);
        thread::sleep(Duration::from_millis(10));

        let elapsed = tracker.elapsed();
        assert!(elapsed.as_millis() >= 10);

        tracker.start();
        thread::sleep(Duration::from_millis(5));

        let new_elapsed = tracker.elapsed();
        assert!(new_elapsed.as_millis() >= 5);
        assert!(new_elapsed < elapsed);
    }

    #[test]
    fn test_time_tracker_eta() {
        let mut tracker = TimeTracker::new(4);

        // No ETA initially
        assert!(tracker.eta().is_none());

        // After completing some tiles
        tracker.tick();
        thread::sleep(Duration::from_millis(10));

        let eta = tracker.eta();
        assert!(eta.is_some());

        // Complete all tiles
        tracker.tick();
        tracker.tick();
        tracker.tick();

        let final_eta = tracker.eta();
        assert!(final_eta.is_some());
        assert_eq!(final_eta.unwrap(), Duration::from_secs(0));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(
            TimeTracker::format_duration(Duration::from_millis(500)),
            "0.500s"
        );
        assert_eq!(
            TimeTracker::format_duration(Duration::from_secs(30)),
            "30.000s"
        );
        assert_eq!(
            TimeTracker::format_duration(Duration::from_secs(90)),
            "1m 30s"
        );
        assert_eq!(
            TimeTracker::format_duration(Duration::from_secs(3665)),
            "1h 01m 05s"
        );
    }

    #[test]
    fn test_progress_calculation() {
        let mut tracker = TimeTracker::new(8);

        assert_eq!(tracker.progress(), 0.0);

        tracker.tick();
        tracker.tick();
        assert_eq!(tracker.progress(), 0.25);

        for _ in 0..6 {
            tracker.tick();
        }
        assert_eq!(tracker.progress(), 1.0);
    }

    #[test]
    fn test_summary() {
        let mut tracker = TimeTracker::new(10);
        tracker.tick();
        tracker.tick();

        let summary = tracker.summary();
        assert!(summary.contains("Completed: 2/10 tiles"));
        assert!(summary.contains("ETA:"));
    }
}
