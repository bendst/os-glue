//! Provide obstractions for embedded OS.

#![feature(alloc)]
#![feature(fnbox)]
#![feature(used)]
#![feature(const_fn)]
#![feature(box_syntax)]
#![no_std]


#[cfg(target_os="none")]
compile_error!(r#"No operating system detected."#);

extern crate embedded_types;
#[cfg(target_os = "riot")]
extern crate riot_sys;
extern crate alloc;


/// Re-export of the underlying bindings.
///
/// Expose constants and bindings.
pub mod raw {
    #[cfg(target_os = "riot")]
    pub use riot_sys::ffi::*;
}

mod sys;

/// Threading abstraction for different os.
///
/// Following OS are available:
/// * RIOT
/// * Every OS supported by rust
///
pub mod thread;

/// Provide syncronizations primitives of the underlying OS.
pub mod sync;


/// Temporal quantification.
pub mod time;

//pub mod net;

#[macro_use]
mod io;
