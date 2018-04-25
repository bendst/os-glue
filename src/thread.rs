// Configure the module which shall be use
// * RIOT
use sys;
pub use sys::Builder;


/// An owned permission to join on a thread (block on its termination).
pub struct JoinHandle(sys::JoinHandle<()>);

impl JoinHandle {
    pub fn thread(&self) -> &sys::Thread {
        self.0.thread()
    }

    #[cfg(all(feature = "riot", target_arch = "arm"))]
    pub fn join(self) {
        self.0.join()
    }
    #[cfg(all(feature = "std", not(target_arch = "arm")))]
    pub fn join(self) {
        self.0.join().map(|_| ()).unwrap()
    }
}

impl From<sys::JoinHandle<()>> for JoinHandle {
    fn from(handle: sys::JoinHandle<()>) -> Self {
        JoinHandle(handle)
    }
}

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// [`JoinHandle`]: struct.JoinHandle.html
pub fn spawn<F>(f: F) -> JoinHandle
where
    F: FnOnce() -> (),
    F: Send + 'static,
{
    sys::spawn::<_, sys::Builder>(f)
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

pub trait BuilderExt
where
    Self: Sized,
{
    type JoinHandle: Send + 'static;

    fn new() -> Self;

    fn name(self, name: &'static str) -> Self;

    fn stack_size(self, _stack_size: i32) -> Self {
        self
    }

    fn priority(self, _priority: u32) -> Self {
        self
    }

    fn flags(self, _flags: i32) -> Self {
        self
    }

    fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, SpawnError>
    where
        F: FnOnce() -> (),
        F: Send + 'static;
}
