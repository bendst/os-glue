use riot_sys::ffi;
use time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct Instant {
    timestamp: ffi::timex_t,
}

impl Instant {
    #[inline]
    pub fn now() -> Instant {
        let timestamp = unsafe {
            let mut timestamp = ffi::timex_set(0, 0);
            ffi::xtimer_now_timex(&mut timestamp as *mut _);
            timestamp
        };
        Instant { timestamp }
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let duration = unsafe { ffi::timex_sub(self.timestamp, earlier.timestamp) };
        // prevent overflowing by saturating
        let nanos = duration.microseconds.saturating_mul(1000);
        Duration::new(duration.seconds.into(), nanos)
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        let now = Instant::now();
        now.duration_since(*self)
    }
}

impl PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.seconds == other.timestamp.seconds && self.timestamp.microseconds == other.timestamp.microseconds
    }
}

impl Eq for Instant {}


use core::cmp::Ordering;

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = (self.timestamp.seconds, self.timestamp.microseconds);
        let rhs = (other.timestamp.seconds, other.timestamp.microseconds);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
