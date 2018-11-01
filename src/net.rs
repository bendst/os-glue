use crate::io;
use crate::sys;

pub use crate::sys::{IpAddress, Ipv4Address, Ipv6Address, SocketAddr};

pub struct UdpSocket(sys::UdpSocket);

#[cfg(not(target_os = "riot"))]
pub const IPV6_LOOPBACK: Ipv6Address = Ipv6Address::LOCALHOST;

#[cfg(not(target_os = "riot"))]
pub const IPV6_UNSPECIFIED: Ipv6Address = Ipv6Address::UNSPECIFIED;

#[cfg(target_os = "riot")]
pub const IPV6_LINK_LOCAL_ALL_ROUTERS: Ipv6Address = Ipv6Address::LINK_LOCAL_ALL_ROUTERS;

#[cfg(target_os = "riot")]
pub const IPV6_LOOPBACK: Ipv6Address = Ipv6Address::LOOPBACK;

#[cfg(target_os = "riot")]
pub const IPV6_UNSPECIFIED: Ipv6Address = Ipv6Address::UNSPECIFIED;

impl UdpSocket {
    pub fn bind<A>(addr: A) -> Result<UdpSocket, io::Error>
    where
        A: Into<sys::SocketAddr>,
    {
        Ok(UdpSocket(sys::UdpSocket::bind(addr.into())?))
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> Result<(usize, sys::SocketAddr), io::Error> {
        self.0.recv_from(buf)
    }

    pub fn send_to<A>(&mut self, buf: &[u8], addr: A) -> Result<usize, io::Error>
    where
        A: Into<sys::SocketAddr>,
    {
        self.0.send_to(buf, addr.into())
    }

    pub fn join_multicast<A>(&mut self, multiaddr: A, interface: u32) -> Result<(), io::Error>
    where
        A: Into<sys::Ipv6Address>,
    {
        self.0.join_multicast_v6(&multiaddr.into(), interface)
    }

    pub fn leave_multicast<A>(&mut self, multiaddr: A, interface: u32) -> Result<(), io::Error>
    where
        A: Into<sys::Ipv6Address>,
    {
        self.0.leave_multicast_v6(&multiaddr.into(), interface)
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), io::Error> {
        self.0.set_nonblocking(nonblocking)
    }
}

pub struct Eui64(pub [u8; 8]);

pub fn eui64() -> Eui64 {
    sys::eui64()
}
