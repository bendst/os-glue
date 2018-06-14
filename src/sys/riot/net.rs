use riot_sys::ffi;
use core::mem;
use core::ptr;
use net::{ErrorKind, IpEndpoint, Ipv6Address, Ipv4Address, IpAddress};


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

        let mut ipv6_local = ffi::SOCK_IPV6_EP_ANY;

        // union access is unsafe
        unsafe { ipv6_local.addr.ipv6.copy_from_slice(local.addr.as_bytes()) };
        ipv6_local.port = local.port;

        let error = unsafe { ffi::sock_udp_create(&mut sock_udp, &ipv6_local, remote, 0) };

        match error {
            0 => {
                let inner = sock_udp;
                Ok(UdpSocket { inner })
            }
            _ => Err(ErrorKind::InvalidInput),
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
            _ => panic!(),
        };

        let endpoint = IpEndpoint::new(addr, remote.port);

        match error {
            size if size < 0 => unimplemented!(),
            size => Ok((size as _, endpoint)),
        }
    }

    #[inline]
    pub fn send<A>(&mut self, buf: &[u8], addr: A) -> Result<usize, ErrorKind>
    where
        A: Into<IpEndpoint>,
    {
        let endpoint: IpEndpoint = addr.into();

        let is_ipv6 = match endpoint.addr {
            IpAddress::Ipv4(..) => false,
            IpAddress::Ipv6(..) => true,
            _ => panic!(),
        };

        let family = if is_ipv6 { ffi::AF_INET6 } else { ffi::AF_INET } as _;

        let mut remote: ffi::sock_udp_ep_t = unsafe { mem::zeroed() };
        remote.family = family;
        remote.port = endpoint.port;

        // union access is unsafe
        if is_ipv6 {
            unsafe { remote.addr.ipv6.copy_from_slice(endpoint.addr.as_bytes()) };
        } else {
            unsafe { remote.addr.ipv4.copy_from_slice(endpoint.addr.as_bytes()) };
        };

        // TODO network interface selection?
        remote.netif = ffi::SOCK_ADDR_ANY_NETIF as _;

        let error = unsafe {
            ffi::sock_udp_send(
                &mut self.inner,
                buf.as_ptr() as *const _,
                buf.len(),
                &remote,
            )
        };

        match error {
            error if error < 0 => unimplemented!(),
            size => Ok(size as _),
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
        use core::ptr;
        let multiaddr = multiaddr.into();
        let error = unsafe { ffi::gnrc_netif_ipv6_group_join(ptr::null(), ptr::null_mut()) };
        unimplemented!()
    }

    #[inline]
    pub fn leave_multicast<A>(&mut self, multiaddr: A, interface: u32) -> Result<(), ErrorKind>
    where
        A: Into<Ipv6Address>,
    {
        let multiaddr = multiaddr.into();
        let error = unsafe { ffi::gnrc_netif_ipv6_group_leave(ptr::null(), ptr::null_mut()) };
        unimplemented!();
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.close();
    }
}

use net;

pub fn eui64() -> net::EUI64 {
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
    net::EUI64(unsafe { eui.uint64.u8 })
}
