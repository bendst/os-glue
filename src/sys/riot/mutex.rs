
use riot_sys::ffi;

use core::mem;
use core::cell::UnsafeCell;

pub struct Mutex(UnsafeCell<ffi::mutex_t>);

impl Mutex {
    pub unsafe fn new() -> Self {
        Mutex(UnsafeCell::new(mem::uninitialized()))
    }

    pub unsafe fn init(&mut self) {
        ffi::mutex_init(self.0.get())
    }

    pub unsafe fn lock(&self) {
        let r = ffi::_mutex_lock(self.0.get(), 1);
        debug_assert_eq!(r, 1);
    }

    pub unsafe fn unlock(&self) {
        ffi::mutex_unlock(self.0.get());
    }

    pub unsafe fn try_lock(&self) -> bool {
        let r = ffi::_mutex_lock(self.0.get(), 0);
        match r {
            1 => true,
            0 => false,
            _ => false,
        }
    }

    pub unsafe fn destroy(&self) {}
}
