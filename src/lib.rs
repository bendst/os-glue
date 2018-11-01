//! Provide obstractions for embedded OS.
//!
//! On tier 1 and 2 targets the os-glue just wraps the underlying stdlib functionality
//! On tier 3 it is hightly dependant on the used IoT operating system
//!
//! For using tier 3 you must always provide a feature flag for particular board.
//!
//! # Currently Supported Tier 3 operating system
//!
//! - RIOT
//!
//!

#![feature(alloc)]
#![feature(fnbox)]
#![cfg_attr(not(target_os = "riot"), feature(const_ip))]
#![feature(const_fn)]
#![feature(box_syntax)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(target_os = "none")]
compile_error!(r#"`os_glue` currently has no target os specified. Thats probably an error."#);

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

//pub mod error;
pub mod net;
/// Temporal quantification.
pub mod time;
#[macro_use]
pub mod io;
