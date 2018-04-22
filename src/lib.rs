#![no_std]

//! Provide obstractions for embedded OS.

#![feature(alloc)]
#![feature(fnbox)]
#![feature(used)]
extern crate alloc;

extern crate embedded_types;

mod sys;

/// Threading abstraction for different os
///
/// Following OS are available:
/// * RIOT
/// * Every OS supported by rust
///
#[cfg(any(feature = "riot", feature = "std"))]
pub mod thread;

#[cfg(any(feature = "riot", feature = "std"))]
pub mod sync;

//#[cfg(any(feature = "riot", feature = "std"))]
//pub mod net;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
