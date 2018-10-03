// Configure the module which shall be use
// * RIOT
use crate::sys;
pub use crate::sys::Thread;
use crate::time;

/// An owned permission to join on a thread (block on its termination).
pub struct JoinHandle(sys::JoinHandle<()>);
pub struct Builder<T>(T);

impl BuilderExt for Builder<sys::Builder> {
    type JoinHandle = <sys::Builder as BuilderExt>::JoinHandle;

    fn new() -> Self {
        Builder(sys::Builder::new())
    }

    fn name(self, name: &'static str) -> Self {
        Builder(<sys::Builder as BuilderExt>::name(self.0, name))
    }

    fn stack_size(self, stack_size: i32) -> Self {
        Builder(<sys::Builder as BuilderExt>::stack_size(self.0, stack_size))
    }

    fn priority(self, priority: u32) -> Self {
        Builder(self.0.priority(priority))
    }

    fn flags(self, flags: i32) -> Self {
        Builder(self.0.flags(flags))
    }

    fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, SpawnError>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        <sys::Builder as BuilderExt>::spawn(self.0, f)
    }
}

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
    sys::spawn::<_, Builder<_>>(f)
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
pub fn park_timeout(duration: time::Duration) {
    sys::park_timeout(duration)
}

/// Puts the current thread to sleep for the specified amount of time.
pub fn sleep(duration: time::Duration) {
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
