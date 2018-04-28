#![no_std]

//! Provide obstractions for embedded OS.

#![feature(alloc)]
#![feature(fnbox)]
#![feature(used)]
#![feature(const_fn)]
#![feature(box_syntax)]
extern crate alloc;

extern crate embedded_types;

#[cfg(feature = "riot")]
extern crate riot_sys;


/// Re-export of the underlying bindings.
///
/// Expose constants and bindings.
pub mod raw {
    #[cfg(feature = "riot")]
    pub use riot_sys::ffi::*;
}


mod sys;

/// Threading abstraction for different os.
///
/// Following OS are available:
/// * RIOT
/// * Every OS supported by rust
///
#[cfg(any(feature = "riot", feature = "std"))]
pub mod thread;

#[cfg(any(feature = "riot", feature = "std"))]
/// Provide syncronizations primitives of the underlying OS.
pub mod sync;


#[cfg(not(feature = "std"))]
pub mod time;

//#[cfg(any(feature = "riot", feature = "std"))]
//pub mod net;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
