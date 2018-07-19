use core::mem;
use core::ptr;
use net::{ErrorKind, IpAddress, IpEndpoint, Ipv4Address, Ipv6Address};
use riot_sys::ffi;

pub struct UdpSocket {
    inner: ffi::sock_udp_t,
}

impl UdpSocket {
    #[inline]
    pub fn create<L>(local: L) -> Result<Self, ErrorKind>
    where
        L: Into<IpEndpoint>,
    {
        let mut sock_udp: ffi::sock_udp_t = unsafe { mem::zeroed() };
        let remote = ptr::null_mut();

        let local = local.into();
        let local = UdpSocket::udp_endpoint(&local);

        let error = unsafe { ffi::sock_udp_create(&mut sock_udp, &local, remote, 0) };

        match error {
            error if error == -(ffi::EADDRINUSE as i32) => Err(ErrorKind::AddrInUse),
            error if error == -(ffi::EAFNOSUPPORT as i32) => Err(ErrorKind::AfNoSupport),
            error if error == -(ffi::EINVAL as i32) => Err(ErrorKind::InvalidInput),
            0 => {
                let inner = sock_udp;
                Ok(UdpSocket { inner })
            }
            _ => unreachable!("Unknown error occured. RIOT API changed."),
        }
    }

    #[inline]
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<(usize, IpEndpoint), ErrorKind> {
        let mut remote: ffi::sock_udp_ep_t = unsafe { mem::zeroed() };
        let error = unsafe {
            ffi::sock_udp_recv(
                &mut self.inner,
                buf.as_mut_ptr() as *mut _,
                buf.len(),
                0,
                &mut remote,
            )
        };

        // union access is unsafe
        let addr = match remote.family as _ {
            ffi::AF_INET6 => {
                let ipv6 = unsafe { remote.addr.ipv6 };
                Ipv6Address::from_bytes(&ipv6).into()
            }
            ffi::AF_INET => {
                let ipv4 = unsafe { remote.addr.ipv4 };
                Ipv4Address::from_bytes(&ipv4).into()
            }
            _ => panic!("Unknown af family"),
        };

        let endpoint = IpEndpoint::new(addr, remote.port);

        match error {
            error if error == -(ffi::EADDRNOTAVAIL as isize) => Err(ErrorKind::AddrMissing),
            error if error == -(ffi::EAGAIN as isize) => Err(ErrorKind::WouldBlock),
            error if error == -(ffi::EINVAL as isize) => Err(ErrorKind::InvalidInput),
            error if error == -(ffi::ENOBUFS as isize) => Err(ErrorKind::BufferToSmall),
            error if error == -(ffi::ENOMEM as isize) => Err(ErrorKind::OutOfMemory),
            error if error == -(ffi::EPROTO as isize) => Err(ErrorKind::Protocol),
            error if error == -(ffi::ETIMEDOUT as isize) => Err(ErrorKind::Timeout),
            size if size >= 0 => Ok((size as _, endpoint)),
            _ => unreachable!("Unknown error occured. RIOT API changed."),
        }
    }

    /// Nice little wrapper for creating udp endpoint for RIOT.
    fn udp_endpoint(endpoint: &IpEndpoint) -> ffi::sock_udp_ep_t {
        let is_ipv6 = match endpoint.addr {
            IpAddress::Ipv4(..) => false,
            IpAddress::Ipv6(..) => true,
            _ => panic!("Unknown address format"),
        };

        let family = if is_ipv6 { ffi::AF_INET6 } else { ffi::AF_INET } as _;

        ffi::sock_udp_ep_t {
            family,
            netif: ffi::SOCK_ADDR_ANY_NETIF as _,
            port: endpoint.port,
            addr: {
                if is_ipv6 {
                    let mut ipv6 = [0; 16];
                    ipv6.copy_from_slice(endpoint.addr.as_bytes());
                    ffi::_sock_tl_ep__bindgen_ty_1 { ipv6 }
                } else {
                    let mut ipv4 = [0; 4];
                    ipv4.copy_from_slice(endpoint.addr.as_bytes());
                    ffi::_sock_tl_ep__bindgen_ty_1 { ipv4 }
                }
            },
        }
    }

    #[inline]
    pub fn send<A>(&mut self, buf: &[u8], addr: A) -> Result<usize, ErrorKind>
    where
        A: Into<IpEndpoint>,
    {
        let endpoint = addr.into();
        let remote = UdpSocket::udp_endpoint(&endpoint);

        let error = unsafe {
            ffi::sock_udp_send(
                &mut self.inner,
                buf.as_ptr() as *const _,
                buf.len(),
                &remote,
            )
        };

        match error {
            error if error == -(ffi::EADDRINUSE as isize) => Err(ErrorKind::AddrInUse),
            error if error == -(ffi::EAFNOSUPPORT as isize) => Err(ErrorKind::AfNoSupport),
            error if error == -(ffi::EHOSTUNREACH as isize) => Err(ErrorKind::HostUnreachable),
            error if error == -(ffi::EINVAL as isize) => Err(ErrorKind::InvalidInput),
            error if error == -(ffi::ENOMEM as isize) => Err(ErrorKind::OutOfMemory),
            error if error == -(ffi::ENOTCONN as isize) => unreachable!("NULL cannot be passed"),
            size if size >= 0 => Ok(size as _),
            _ => unreachable!("Unknown error occurred. RIOT API changed."),
        }
    }

    #[inline]
    pub fn close(&mut self) {
        unsafe { ffi::sock_udp_close(&mut self.inner) }
    }

    #[inline]
    pub fn join_multicast<A>(&mut self, multiaddr: A, interface: u32) -> Result<(), ErrorKind>
    where
        A: Into<Ipv6Address>,
    {
        let interface = find_interface(interface).ok_or(ErrorKind::NoMatchingInterface)?;

        let multiaddr = multiaddr.into();
        let mut addr_buffer = [0; 16];
        addr_buffer.copy_from_slice(multiaddr.as_bytes());

        let mut multiaddr = ffi::ipv6_addr_t { u8: addr_buffer };

        let error = unsafe { ffi::gnrc_netif_ipv6_group_join(interface, &mut multiaddr) };

        match error {
            error if error == -(ffi::ENOMEM as i32) => Err(ErrorKind::OutOfMemory),
            error if error == -(ffi::ENOTSUP as i32) => Err(ErrorKind::NotSupported),
            size if size == mem::size_of::<ffi::ipv6_addr_t>() as _ => Ok(()),
            _ => unreachable!("Unknown error occurred. RIOT API changed"),
        }
    }

    #[inline]
    pub fn leave_multicast<A>(&mut self, multiaddr: A, interface: u32) -> Result<(), ErrorKind>
    where
        A: Into<Ipv6Address>,
    {
        let interface = find_interface(interface).ok_or(ErrorKind::NoMatchingInterface)?;

        let multiaddr = multiaddr.into();
        let mut addr_buffer = [0; 16];
        addr_buffer.copy_from_slice(multiaddr.as_bytes());

        let mut multiaddr = ffi::ipv6_addr_t { u8: addr_buffer };
        let error = unsafe { ffi::gnrc_netif_ipv6_group_leave(interface, &mut multiaddr) };

        match error {
            error if error == -(ffi::ENOTSUP as i32) => Err(ErrorKind::NotSupported),
            size if size == mem::size_of::<ffi::ipv6_addr_t>() as _ => Ok(()),
            _ => unreachable!("Unknown error occured. RIOT API changed"),
        }
    }
}

fn find_interface(mut index: u32) -> Option<*mut ffi::gnrc_netif_t> {
    let mut next = ptr::null();
    while let Some(interface) = unsafe { ffi::gnrc_netif_iter(next) }.into() {
        if index == 0 {
            return Some(interface);
        }
        next = interface;
        index -= 1;
    }
    None
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.close();
    }
}

use net;

pub fn eui64() -> net::Eui64 {
    let mut eui = ffi::eui64_t { uint8: [0; 8] };

    unsafe {
        let netif = ffi::gnrc_netif_iter(ptr::null_mut());
        ffi::netdev_eth_get(
            (*netif).dev,
            ffi::netopt_t_NETOPT_IPV6_IID,
            &mut eui as *mut ffi::eui64_t as *mut _,
            mem::size_of::<ffi::eui64_t>(),
        );
    };

    // union access eui is always 64-bit
    net::Eui64(unsafe { eui.uint64.u8 })
}
