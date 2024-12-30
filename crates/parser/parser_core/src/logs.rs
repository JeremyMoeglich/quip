#[cfg(feature = "log")]
macro_rules! log {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
macro_rules! log {
    ($($arg:tt)*) => {};
}
