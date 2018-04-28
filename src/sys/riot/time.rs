use riot_sys::ffi;
use time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct Instant {
    timestamp: ffi::timex_t,
}

impl Instant {
    #[inline(always)]
    pub fn now() -> Instant {
        let timestamp = unsafe {
            let mut timestamp = ffi::timex_set(0, 0);
            ffi::xtimer_now_timex(&mut timestamp as *mut _);
            timestamp
        };
        Instant { timestamp }
    }

    #[inline(always)]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let duration = unsafe { ffi::timex_sub(self.timestamp, earlier.timestamp) };
        // prevent overflowing by saturating
        let nanos = duration.microseconds.saturating_mul(1000);
        Duration::new(duration.seconds.into(), nanos)
    }

    #[inline(always)]
    pub fn elapsed(&self) -> Duration {
        let now = Instant::now();
        now.duration_since(*self)
    }
}
