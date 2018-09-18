use core::fmt;
use core::fmt::Write;
use crate::sync::Mutex;
use riot_sys::ffi;

struct Writer;
struct SyncWriter(Mutex<Writer>);

static WRITER: SyncWriter = SyncWriter(Mutex::new(Writer));

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::print(s.as_ptr(), s.len());
        }
        Ok(())
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut writer = WRITER.0.lock();
    writer.write_fmt(args).unwrap()
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    OutOfMemory,
    AddrInUse,
    AddrMissing,
    BufferToSmall,
    AfNoSupport,
    NotSupported,
    InvalidInput,
    Protocol,
    WouldBlock,
    Timeout,
    HostUnreachable,
    NoMatchingInterface,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self.kind)
    }
}
