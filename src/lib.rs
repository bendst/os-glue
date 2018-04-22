#![no_std]

//! Provide obstractions for embedded OS.

#![feature(alloc)]
#![feature(fnbox)]
#![feature(used)]
extern crate alloc;


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

fn t() {
    use self::thread::BuilderExt;
    use self::thread::Builder;

    let a : Builder = BuilderExt::<()>::new();

    a.spawn(|| ());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
