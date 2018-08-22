use alloc::boxed::Box;
use alloc::boxed::FnBox;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ptr;
pub use core::time::Duration;
use crate::thread;
use crate::thread::BuilderExt;

use riot_sys::ffi;

pub struct JoinHandle<T = ()> {
    thread: Thread,
    #[used]
    stack_buffer: Vec<u8>,
    result: Option<()>,
    _marker: PhantomData<T>,
}

impl<T> JoinHandle<T> {
    #[inline]
    pub fn thread(&self) -> &Thread {
        &self.thread
    }

    #[inline]
    pub fn join(self) -> () {
        let mut this = self;
        this.result.take().unwrap()
    }
}

/// A handle to a thread
pub struct Thread {
    id: ThreadId,
}

impl Thread {
    #[inline]
    pub fn unpark(&self) {
        unsafe {
            let _ = ffi::thread_wakeup(self.id.0);
        }
    }

    #[inline]
    pub fn id(&self) -> ThreadId {
        self.id
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ThreadId(ffi::kernel_pid_t);

#[inline]
pub fn current() -> Thread {
    // RIOT does it the same way with an inlined function.
    let id = unsafe { ptr::read_volatile(&ffi::sched_active_pid as *const _) };
    Thread { id: ThreadId(id) }
}

#[inline]
pub fn sleep(_duration: Duration) {
    // TODO
    unimplemented!()
}

#[inline]
pub fn park() {
    unsafe { ffi::thread_sleep() }
}

#[inline]
pub fn panicking() -> bool {
    false
}

#[inline]
pub fn park_timeout(_duration: Duration) {
    unimplemented!("RIOT does not support timeouts")
}

#[inline]
unsafe fn spawn_inner<'a>(
    f: Box<FnBox() -> () + Send + 'a>,
    name: &'static str,
    stack_size: i32,
    flags: i32,
    priority: u32,
) -> Result<JoinHandle<()>, thread::SpawnError> {
    // Directly allocate our 'heap'
    let f = box f;

    // extract the parameters, which will be the environment of the closure
    let param_ptr = &*f as *const _ as *mut _;

    let mut buffer = Vec::with_capacity(stack_size as usize);

    let id = ffi::thread_create(
        buffer.as_mut_ptr(),
        stack_size,
        priority as _,
        flags,
        Some(thread_start),
        param_ptr,
        name.as_ptr(),
    );

    assert!(id > 0, "thread id is invalid");

    extern "C" fn thread_start(main: *mut ffi::c_void) -> *mut ffi::c_void {
        unsafe { Box::from_raw(main as *mut Box<FnBox()>)() }
        ptr::null_mut()
    }

    Ok(JoinHandle {
        _marker: PhantomData,
        thread: Thread { id: ThreadId(id) },
        stack_buffer: buffer,
        result: None,
    })
}

#[inline]
pub fn spawn<F, B>(f: F) -> B::JoinHandle
where
    F: FnOnce(),
    F: Send + 'static,
    B: BuilderExt,
{
    B::new().spawn(f).expect("thread spawn failed")
}

#[inline]
pub fn yield_now() {
    unsafe { ffi::thread_yield() }
}

#[derive(Default)]
pub struct Builder {
    name: Option<&'static str>,
    stack_size: Option<i32>,
    priority: Option<u32>,
    flags: Option<i32>,
}

impl BuilderExt for Builder {
    type JoinHandle = thread::JoinHandle;
    #[inline]
    fn new() -> Self {
        Builder {
            ..Default::default()
        }
    }

    #[inline]
    fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    #[inline]
    fn stack_size(mut self, stack_size: i32) -> Self {
        self.stack_size = Some(stack_size);
        self
    }

    #[inline]
    fn priority(mut self, priority: u32) -> Self {
        self.priority = Some(priority);
        self
    }

    #[inline]
    fn flags(mut self, flags: i32) -> Self {
        self.flags = Some(flags);
        self
    }

    #[inline]
    fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, thread::SpawnError>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        let Builder {
            name,
            stack_size,
            flags,
            priority,
        } = self;

        let name = name.unwrap_or("rust_thread");
        let stack_size = stack_size.unwrap_or(512);
        let flags = flags.unwrap_or(0);
        // TODO probably should warn about the default behaviour
        let priority = priority.unwrap_or(ffi::THREAD_PRIORITY_MAIN - 1);

        unsafe { spawn_inner(box f, name, stack_size, flags, priority).map(From::from) }
    }
}
