
use riot_sys::ffi;

use core::ptr;
use core::cell::UnsafeCell;

pub struct Mutex(UnsafeCell<ffi::mutex_t>);

impl Mutex {
    pub const unsafe fn new() -> Self {
        Mutex(UnsafeCell::new(ffi::mutex_t {
            queue: ffi::list_node { next: ptr::null_mut() },
        }))
    }

    #[allow(unused)]
    pub unsafe fn init(&mut self) {
        //ffi::mutex_init(self.0.get())
    }

    pub unsafe fn lock(&self) {
        ffi::mutex_lock(self.0.get());
    }

    pub unsafe fn unlock(&self) {
        ffi::mutex_unlock(self.0.get());
    }

    pub unsafe fn try_lock(&self) -> bool {
        let r = ffi::mutex_trylock(self.0.get());
        match r {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    pub unsafe fn destroy(&self) {}
}
