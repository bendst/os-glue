
use core::ops::Deref;
use core::ops::DerefMut;
use core::cell::UnsafeCell;

use sys;

/// Mutual exclusion primitive.
pub struct Mutex<T: ?Sized> {
    // TODO check whether the Mutex should be allocated on the heap or can be safely inlined.
    lock: sys::Mutex,
    data: UnsafeCell<T>,
}

/// An enumeration of possible erros which can occur while trying to acquire a lock, from the
/// [try_lock] method on a [Mutex]
///
/// [try_lock]: Mutex::try_lock
pub enum TryLock {
    /// The lock could not be acquired at this time because the operation would otherwise block.
    WouldBlock,
}

/// An RAII implmentation of a 'scoped lock' of a mutex. When this structure is dropped, the lock
/// will be unlocked
///
/// The data can accessed through this guard via its [Deref] and [DerefMut] implementations.
///
/// This structure is creaty by the [lock] and [try_lock] methods on [Mutex]
///
/// [lock]: Mutex::lock
/// [try_lock]: Mutex::try_lock
/// [Deref]: https://doc.rust-lang.org/core/ops/trait.Deref.html
/// [DerefMut]: https://doc.rust-lang.org/core/ops/trait.DerefMut.html
pub struct MutexGuard<'lock, T: ?Sized + 'lock> {
    inner: &'lock Mutex<T>,
}

impl<T> Mutex<T> {
    /// Create a new unlocked mutex.
    pub fn new(data: T) -> Self {
        let lock = unsafe {
            let mut lock = sys::Mutex::new();
            lock.init();
            lock
        };

        Mutex {
            lock,
            data: UnsafeCell::new(data),
        }
    }
}

impl<'a, T: ?Sized> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.destroy();
        }
    }
}

impl<'a, T: ?Sized> Mutex<T> {
    /// Acquire a mutex, blocking the current thread.
    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            self.lock.lock();
        }
        MutexGuard::new(self)
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired, then [TryLock] is returned. Otherwise, an RAII guard is
    /// returned.
    pub fn try_lock(&self) -> Result<MutexGuard<T>, TryLock> {
        if unsafe { self.lock.try_lock() } {
            Ok(MutexGuard::new(self))
        } else {
            Err(TryLock::WouldBlock)
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<'lock, T: ?Sized> MutexGuard<'lock, T> {
    fn new(mutex: &'lock Mutex<T>) -> Self {
        MutexGuard { inner: mutex }
    }
}

impl<'lock, T: ?Sized> Deref for MutexGuard<'lock, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.data.get() }
    }
}

impl<'lock, T: ?Sized> DerefMut for MutexGuard<'lock, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.data.get() }
    }
}

impl<'lock, T: ?Sized> Drop for MutexGuard<'lock, T> {
    fn drop(&mut self) {
        unsafe {
            self.inner.lock.unlock();
        }
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(data: T) -> Self {
        Mutex::new(data)
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(Default::default())
    }
}
