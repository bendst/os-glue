pub use core::time::Duration;

use sys;

/// A measurment of a monotonically nondecreasing clock. Opaque and useful only with [Duration].
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Instant(sys::Instant);

impl Instant {
    /// Returns the instant corresponding to 'now'.
    ///
    /// # Examples
    /// ```
    /// use os_glue::time::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    pub fn now() -> Instant {
        Instant(sys::Instant::now())
    }

    /// Returns the amount of time elasped from another instant to this one.
    ///
    /// # Panics
    /// This function can panic in some cases when `earlier` is later than self.
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }

    /// Returns the amount of time elasped since this instant was created.
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}
