#[cfg(feature = "std")]
mod std_x86_64 {
    use crate::net;

    use crate::std::fmt;
    pub use crate::std::io::{Error, ErrorKind};
    pub use crate::std::net::{
        IpAddr as IpAddress, Ipv4Addr as Ipv4Address, Ipv6Addr as Ipv6Address, SocketAddr,
        UdpSocket,
    };
    use crate::std::ops::{Add, Sub};
    pub(crate) use crate::std::thread::Builder;
    pub use crate::std::thread::{
        current, panicking, park, park_timeout, sleep, yield_now, JoinHandle, Thread,
    };
    pub use crate::std::time::Duration;

    use crate::thread;

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

    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn _print(args: fmt::Arguments) {
        use crate::std::io;
        use crate::std::io::Write;

        let stdout = io::stdout();
        let mut guard = stdout.lock();

        guard.write_fmt(args).unwrap()
    }

    pub fn eui64() -> net::Eui64 {
        use mac_address::get_mac_address;

        let mac = get_mac_address()
            .expect("Unable to fetch MAC address.")
            .expect("No mac address found");

        let mac_bytes = mac.bytes();

        let mut eui64 = [0xFF; 8];

        // Split the MAC address
        eui64[..3].copy_from_slice(&mac_bytes[..3]);
        eui64[5..].copy_from_slice(&mac_bytes[3..]);

        // invert the universal/local (U/L) flag (bit 7) in the OUI portion of the address
        eui64[0] ^= 0x02;

        net::Eui64(eui64)
    }

    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
    pub struct Instant {
        timespec: time::Timespec,
    }

    impl Instant {
        #[inline]
        pub fn now() -> Self {
            let tm = time::now();
            let timespec = tm.to_timespec();
            Instant { timespec }
        }

        #[inline]
        pub fn duration_since(&self, earlier: Instant) -> Duration {
            let duration = self.timespec - earlier.timespec;
            duration.to_std().unwrap()
        }

        #[inline]
        pub fn elapsed(&self) -> Duration {
            let now = Instant::now();
            now.duration_since(*self)
        }
    }

    impl From<(i32, u32)> for Instant {
        fn from((sec, nsec): (i32, u32)) -> Self {
            Instant {
                timespec: time::Timespec::new(i64::from(sec), nsec as _),
            }
        }
    }

    impl Sub<crate::time::Duration> for Instant {
        type Output = Instant;
        fn sub(self, other: Duration) -> Self::Output {
            Instant {
                timespec: self.timespec - time::Duration::from_std(other).unwrap(),
            }
        }
    }

    impl Add<crate::time::Duration> for Instant {
        type Output = Instant;
        fn add(self, other: Duration) -> Self::Output {
            Instant {
                timespec: self.timespec + time::Duration::from_std(other).unwrap(),
            }
        }
    }
}

#[cfg(feature = "std")]
#[allow(unused_imports)]
pub use self::std_x86_64::_print;
#[cfg(feature = "std")]
pub use self::std_x86_64::*;

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
    riot => thread, net, mutex, time, io
}

#[cfg(target_os = "riot")]
#[allow(unused_imports)]
#[doc(hidden)]
pub use self::riot::io::_print;
