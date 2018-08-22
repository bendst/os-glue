//! Provide obstractions for embedded OS.

#![feature(alloc)]
#![feature(fnbox)]
#![feature(used)]
#![feature(const_fn)]
#![feature(box_syntax)]
#![no_std]

#[cfg(target_os = "none")]
compile_error!(r#"`os_glue` currently has no target os specified. Thats probably an error."#);

extern crate alloc;
extern crate embedded_types;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
extern crate mac_address;

//#[cfg(feature = "std")]
//extern crate time as time2;

#[cfg(feature = "std")]
extern crate spin;

#[cfg(target_os = "riot")]
extern crate riot_sys;

extern crate smoltcp;

/// Re-export of the underlying bindings.
///
/// Expose constants and bindings.
pub mod raw {
    #[cfg(target_os = "riot")]
    pub use riot_sys::ffi::*;
}

pub(crate) mod sys;

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

pub mod net;

pub mod error;

#[macro_use]
pub mod io;
