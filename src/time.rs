pub use core::time::Duration;

use sys;

#[derive(Debug, Copy, Clone)]
pub struct Instant(sys::Instant);

impl Instant {
    pub fn now() -> Instant {
        Instant(sys::Instant::now())
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}
