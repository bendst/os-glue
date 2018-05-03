use riot_sys::ffi;

struct UdpSocket {
    inner: ffi::socket_udp_t,
}

impl UdpSocket {
    pub fn new() -> Self {
        let mut sock_udp : ffi::sock_udp_t = mem::zeroed();
        ffi::socket_udp_create(&mut socket_udp as *mut _);
        UdpSocket {
            inner
        }
    }
}
