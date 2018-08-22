// Only provide mutex for 'embedded'. In any other case just use the mutexes provided by rust.
#[cfg(target_os = "riot")]
mod mutex;

#[cfg(target_os = "riot")]
pub use self::mutex::*;

#[cfg(feature = "std")]
pub use spin::{Mutex, MutexGuard};
