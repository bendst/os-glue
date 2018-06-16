// Only provide mutex for 'embedded'. In any other case just use the mutexes provided by rust.
#[cfg(target_os="riot")]
mod mutex;

#[cfg(target_os="riot")]
pub use self::mutex::*;

// # TODO 
// Implement a alternative Mutex via spinlocks, 
// because the std lib does not provide a mutex which
// can be initialized const.

