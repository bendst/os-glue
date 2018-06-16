
#[cfg(all(not(target_arch = "arm"), feature = "std"))]
mod std_x86_64 {
    extern crate std;
    pub use self::std::thread::{Builder, yield_now, sleep, park_timeout, park, panicking, current,
                                Thread, JoinHandle};
    pub use self::std::sync::Mutex;
    pub use self::std::time::Duration;
    pub use self::std::time::Instant;
    pub use self::std::net::UdpSocket;

    use thread;

    impl thread::BuilderExt for Builder {
        type JoinHandle = thread::JoinHandle;
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
            F: FnOnce() -> (),
            F: Send + 'static,
        {
            Builder::spawn(self, f)
                .map_err(|_| thread::SpawnError::SpawnFailed)
                .map(From::from)
        }
    }

    pub fn spawn<F, B>(f: F) -> B::JoinHandle
    where
        F: FnOnce() -> (),
        F: Send + 'static,
        B: thread::BuilderExt,
    {
        B::new().spawn(f).expect("thread spawn failed")
    }

    use self::std::fmt;

    #[allow(dead_code)]
    pub(crate) fn print(args: fmt::Arguments) {
        use self::std::io::Write;
        use self::std::io;

        let stdout = io::stdout();
        let mut guard = stdout.lock();

        guard.write_fmt(args).unwrap()
    }
}

#[cfg(feature = "std")]
pub use self::std_x86_64::*;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use self::std_x86_64::print;


#[cfg(target_os = "riot")]
mod riot;


macro_rules! pub_use {
    ($meta: meta, $os: ident => $( $item: ident ),*) => {
        $(
            #[cfg($meta)]
            pub use self::$os::$item::*;
        )* 
    };
}

pub_use! {
    target_os = "riot",
    riot => thread, net, mutex, time
}

#[cfg(target_os = "riot")]
#[allow(unused_imports)]
pub(crate) use self::riot::io::print;
