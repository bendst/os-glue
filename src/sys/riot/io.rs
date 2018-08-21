use core::fmt;
use core::fmt::Write;
use riot_sys::ffi;
use sync::Mutex;

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
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
