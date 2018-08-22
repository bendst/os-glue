#[doc(hidden)]
pub use crate::sys::_print;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! println {
    () => {
        print!("\n")
    };
    ($fmt: expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt: expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*);
    }
}

pub use crate::sys::Error;
