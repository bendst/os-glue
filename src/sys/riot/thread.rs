pub use core::time::Duration;
use core::marker::PhantomData;
use alloc::boxed::FnBox;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::ptr;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod ffi {
    pub type kernel_pid_t = i16;
    pub type thread_task_func_t = extern "C" fn(*mut u8) -> *mut u8;

    extern "C" {
        pub static sched_active_pid: kernel_pid_t;
        pub fn thread_yield();
        pub fn thread_sleep();
        pub fn thread_wakeup(id: kernel_pid_t) -> i32;
        pub fn thread_create(
            stack: *mut u8,
            stacksize: i32,
            priority: u8,
            flags: i32,
            task_func: thread_task_func_t,
            arg: *const u8,
            name: *const u8,
        ) -> kernel_pid_t;
    }
}

pub struct JoinHandle<T> {
    _marker: PhantomData<T>,
    thread: Thread,
    #[used]
    stack_buffer: Vec<u8>,
    result: Option<T>,
}

impl<T> JoinHandle<T> {
    pub fn thread(&self) -> &Thread {
        &self.thread
    }

    pub fn join(self) -> T {
        let mut this = self;
        this.result.take().unwrap()
    }
}

/// A handle to a thread
pub struct Thread {
    id: ThreadId,
}

impl Thread {
    pub fn unpark(&self) {
        unsafe {
            let _ = ffi::thread_wakeup(self.id.0);
        }
    }

    pub fn id(&self) -> ThreadId {
        self.id
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ThreadId(ffi::kernel_pid_t);


pub fn current() -> Thread {
    // RIOT does it the same way with an inlined function.
    let id = unsafe { ptr::read_volatile(&ffi::sched_active_pid as *const _) };
    Thread { id: ThreadId(id) }
}

pub fn sleep(_duration: Duration) {
    unimplemented!("RIOT does not support sleeps")
}

pub fn park() {
    unsafe { ffi::thread_sleep() }
}

pub fn panicking() -> bool {
    false
}

pub fn park_timeout(_duration: Duration) {
    unimplemented!("RIOT does not support timeouts")
}


unsafe fn spawn_inner<'a, T>(
    f: Box<FnBox() -> T + Send + 'a>,
    name: &'static str,
    stack_size: i32,
    flags: i32,
    priority: u8,
) -> Result<JoinHandle<T>, thread::SpawnError>
where
    T: Send + 'static,
{
    let f = Box::new(f);
    let param_ptr = &*f as *const _ as *mut _;

    let mut buffer = Vec::with_capacity(stack_size as usize);

    let id = ffi::thread_create(
        buffer.as_mut_ptr(),
        stack_size,
        priority,
        flags,
        thread_start,
        param_ptr,
        name.as_ptr(),
    );

    extern "C" fn thread_start(main: *mut u8) -> *mut u8 {
        unsafe {
            let b = Box::from_raw(main as *mut Box<FnBox()>);
            b()
        }
        ptr::null_mut()
    }

    Ok(JoinHandle {
        _marker: PhantomData,
        thread: Thread { id: ThreadId(id) },
        stack_buffer: buffer,
        result: None,
    })
}

pub fn spawn<F, B, T>(f: F) -> B::JoinHandle
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
    B: BuilderExt<T>,
{
    B::new().spawn(f).expect("thread spawn failed")
}


pub fn yield_now() {
    unsafe { ffi::thread_yield() }
}

#[derive(Default)]
pub struct Builder {
    name: Option<&'static str>,
    stack_size: Option<i32>,
    priority: Option<u8>,
    flags: Option<i32>,
}

use thread::BuilderExt;
use thread;


impl<T> BuilderExt<T> for Builder
where
    T: Send + 'static,
{
    type JoinHandle = thread::JoinHandle<T>;
    fn new() -> Self {
        Builder { ..Default::default() }
    }
    fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    fn stack_size(mut self, stack_size: i32) -> Self {
        self.stack_size = Some(stack_size);
        self
    }

    fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    fn flags(mut self, flags: i32) -> Self {
        self.flags = Some(flags);
        self
    }

    fn spawn<F>(self, f: F) -> Result<Self::JoinHandle, thread::SpawnError>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let Builder {
            name,
            stack_size,
            flags,
            priority,
        } = self;

        let name = name.unwrap_or("rust_thread");
        let stack_size = stack_size.unwrap_or(256);
        let flags = flags.unwrap(); // TODO
        let priority = priority.unwrap(); //   TODO

        unsafe { spawn_inner(Box::new(f), name, stack_size, flags, priority).map(From::from) }
    }
}
