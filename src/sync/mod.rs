#[cfg(all(target_arch = "arm", not(feature = "std"), any(feature = "riot")))]
mod mutex;

#[cfg(all(not(target_arch = "arm"), feature = "std"))]
pub use sys::*;

#[cfg(all(target_arch = "arm", not(feature = "std"), any(feature = "riot")))]
pub use self::mutex::*;
