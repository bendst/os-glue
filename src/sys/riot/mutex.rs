use riot_sys::ffi;

use core::cell::UnsafeCell;
use core::ptr;

pub struct Mutex(UnsafeCell<ffi::mutex_t>);

unsafe impl Sync for Mutex {}
unsafe impl Send for Mutex {}

impl Mutex {
    pub const unsafe fn new() -> Self {
        Mutex(UnsafeCell::new(ffi::mutex_t {
            queue: ffi::list_node {
                next: ptr::null_mut(),
            },
        }))
    }

    #[allow(unused)]
    #[inline]
    pub unsafe fn init(&self) {
        //ffi::mutex_init(self.0.get())
        unimplemented!("No special initialization needed")
    }

    #[inline]
    pub unsafe fn lock(&self) {
        ffi::mutex_lock(self.0.get());
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        ffi::mutex_unlock(self.0.get());
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        let r = ffi::mutex_trylock(self.0.get());
        match r {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        unimplemented!("RIOT has no destroy function.")
    }
}
