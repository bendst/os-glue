use core::fmt;
use core::fmt::Write;
use sync::Mutex;
use riot_sys::ffi;

struct Writer;
struct SyncWriter(Mutex<Writer>);

static WRITER: SyncWriter = SyncWriter(Mutex::new(Writer));

impl fmt::Write for Writer {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::print(s.as_ptr(), s.len());
        }
        Ok(())
    }
}

#[inline(always)]
pub(crate) fn print(args: fmt::Arguments) {
    let mut writer = WRITER.0.lock();
    writer.write_fmt(args).unwrap()
}