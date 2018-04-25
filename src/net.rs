use embedded_types::io::Write;
use embedded_types::io::Read;
use embedded_types::io;

use core::mem;
use core::slice;
use core::usize;

pub struct RawIoVec {
    ptr: *const u8,
    len: usize,
}

pub struct IoVec {
    inner: [u8],
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts(vec.ptr as *const u8, vec.len)
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts_mut(vec.ptr as *mut u8, vec.len)
        }
    }

    unsafe fn iovec(&self) -> RawIoVec {
        mem::transmute(&self.inner)
    }
}

impl<'a> From<&'a [u8]> for &'a IoVec {
    fn from(src: &'a [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(RawIoVec {
                ptr: src.as_ptr() as *mut _,
                len: src.len(),
            })
        }
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(RawIoVec {
                ptr: src.as_ptr() as *mut _,
                len: src.len(),
            })
        }
    }
}

pub trait Network: Write + Read {
    type Error;
    fn send<R>(&mut self, buffer: Read) -> Result<usize, Self::Error>;
    fn receive<W: Write>(&mut self, buffer: W);
}

impl Write for IoVec {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use core::cmp;
        use core::mem;

        let this = &mut self.as_mut();

        let amt = cmp::min(buf.len(), this.len());
        let (a, b) = mem::replace(this, &mut []).split_at_mut(amt);
        a.copy_from_slice(&buf[..]);
        *this = b;
        Ok(amt)
    }
}