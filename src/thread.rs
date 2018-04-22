// Configure the module which shall be use
// * RIOT
use sys;
pub use sys::Builder;


/// An owned permission to join on a thread (block on its termination).
pub struct JoinHandle<T>(sys::JoinHandle<T>);

impl<T> JoinHandle<T> {
    pub fn thread(&self) -> &sys::Thread {
        self.0.thread()
    }

    #[cfg(all(feature = "riot", target_arch = "arm"))]
    pub fn join(self) -> T {
        self.0.join()
    }
    #[cfg(all(feature = "std", not(target_arch = "arm")))]
    pub fn join(self) -> T {
        self.0.join().unwrap()
    }
}

impl<T> From<sys::JoinHandle<T>> for JoinHandle<T> {
    fn from(handle: sys::JoinHandle<T>) -> Self {
        JoinHandle(handle)
    }
}

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// [`JoinHandle`]: struct.JoinHandle.html
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    sys::spawn::<_, sys::Builder, _>(f)
}

/// Gets a handle to the thread that invokes it.
pub fn current() -> sys::Thread {
    sys::current()
}

/// Determines whether the current thread is unwinding because of panic.
pub fn panicking() -> bool {
    sys::panicking()
}

/// Blocks unless or until the current thread's token is made available.
pub fn park() {
    sys::park()
}

/// Blocks unless or until the current thread's token is made available or
/// the specified duration has been reached (may wake spuriously).
pub fn park_timeout(duration: sys::Duration) {
    sys::park_timeout(duration)
}

/// Puts the current thread to sleep for the specified amount of time.
pub fn sleep(duration: sys::Duration) {
    sys::sleep(duration)
}

/// Cooperatively gives up a timeslice to the OS scheduler.
pub fn yield_now() {
    sys::yield_now()
}

#[derive(Debug)]
pub enum SpawnError {
    SpawnFailed,
}

pub trait BuilderExt<T>
where
    T: Send + 'static,
{
    type JoinHandle: Send + 'static;

    fn new() -> Self;
    fn name(self, name: &'static str) -> Self;
    fn stack_size(self, stack_size: i32) -> Self;
    fn priority(self, priority: u8) -> Self;
    fn flags(self, flags: i32) -> Self;
    fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, SpawnError>
    where
        F: FnOnce() -> T,
        F: Send + 'static;
}
