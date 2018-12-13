//! Provide abstractions for embedded OS.
//!
//! os-glue wraps for tier 1 and 2 the underlying standard library functionality.
//! For tier 3 it attempts to provide similaiar functionality, but it is hightly dependant on the OS.
//!
//! You must provide a feature flag for the board for tier 3.
//!
//! # Supported Tier 3 OS
//!
//! - RIOT
//!
//!

#![feature(alloc)]
#![feature(fnbox)]
#![cfg_attr(not(target_os = "riot"), feature(const_ip))]
#![feature(const_fn)]
#![feature(box_syntax)]
#![feature(extern_crate_item_prelude)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

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
