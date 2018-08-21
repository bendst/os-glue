#[cfg(all(not(target_arch = "arm"), feature = "std"))]
mod std_x86_64 {
    use net;
    extern crate std;
    pub use self::std::io::Error;
    pub use self::std::net::{
        IpAddr as IpAddress, Ipv4Addr as Ipv4Address, Ipv6Addr as Ipv6Address, SocketAddr,
        UdpSocket,
    };
    pub use self::std::sync::{Mutex, MutexGuard};
    pub use self::std::thread::{
        current, panicking, park, park_timeout, sleep, yield_now, Builder, JoinHandle, Thread,
    };
    pub use self::std::time::Duration;
    pub use self::std::time::Instant;

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
    #[doc(hidden)]
    pub fn _print(args: fmt::Arguments) {
        use self::std::io;
        use self::std::io::Write;

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
