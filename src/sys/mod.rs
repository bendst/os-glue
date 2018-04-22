#[cfg(all(target_arch = "arm", feature = "riot"))]
mod riot;

#[cfg(all(not(target_arch = "arm"), feature = "std"))]
mod std_x86_64 {
    extern crate std;
    pub use self::std::thread::{Builder, yield_now, sleep, park_timeout, park, panicking, current,
                                Thread, JoinHandle};
    pub use self::std::sync::Mutex;
    pub use self::std::time::Duration;

    use thread;

    impl<T> thread::BuilderExt<T> for Builder
    where
        T: Send + 'static,
    {
        type JoinHandle = thread::JoinHandle<T>;
        fn new() -> Self {
            Builder::new()
        }

        fn name(self, name: &'static str) -> Self {
            Builder::name(self, name.into())
        }

        fn stack_size(self, stack_size: i32) -> Self {
            Builder::stack_size(self, stack_size as _)
        }
        fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, thread::SpawnError>
        where
            F: FnOnce() -> T,
            F: Send + 'static,
        {
            Builder::spawn(self, f)
                .map_err(|_| thread::SpawnError::SpawnFailed)
                .map(From::from)
        }
    }

    pub fn spawn<F, B, T>(f: F) -> B::JoinHandle
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
        B: thread::BuilderExt<T>,
    {
        B::new().spawn(f).expect("thread spawn failed")
    }
}

#[cfg(all(target_arch = "arm", feature = "riot"))]
pub use self::riot::thread::*;
#[cfg(all(target_arch = "arm", feature = "riot"))]
pub use self::riot::mutex::*;


#[cfg(all(not(target_arch = "arm"), feature = "std"))]
pub use self::std_x86_64::*;
