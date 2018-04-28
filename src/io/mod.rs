#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::sys::print(format_args!($($arg)*))
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
