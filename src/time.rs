pub use core::time::Duration;

use sys;
use core::ops::{Add, Sub};

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

impl<T> From<T> for Instant
where
    T: Into<sys::Instant>,
{
    fn from(t: T) -> Self {
        Instant(t.into())
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, other: Instant) -> Self::Output {
        self.duration_since(other)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, other: Duration) -> Self::Output {
        Instant(self.0 - other)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, other: Duration) -> Self::Output {
        Instant(self.0 + other)
    }
}
