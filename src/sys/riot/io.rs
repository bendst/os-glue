use core::fmt;
use core::fmt::Write;
use sync::Mutex;
use riot_sys::ffi;

struct Writer;
struct SyncWriter(Option<Mutex<Writer>>);

static mut WRITER: SyncWriter = SyncWriter(None);

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::print(s.as_ptr(), s.len());
        }
        Ok(())
    }
}

#[inline(always)]
pub(crate) fn print(args: fmt::Arguments) {
    // While still very unliky, we must initialize Mutex with Writer unsafely once,
    // because the Mutex creation is not constant.
    let mut writer = unsafe { WRITER.0.get_or_insert(Mutex::new(Writer)) }.lock();
    writer.write_fmt(args).unwrap()
}
