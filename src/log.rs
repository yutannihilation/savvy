#![allow(unused_macros)]
#![allow(unused_imports)]

#[cfg(feature = "logger")]
macro_rules! debug {
    ($($arg:tt)+) => (log::debug!($($arg)+))
}

#[cfg(not(feature = "logger"))]
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(feature = "logger")]
macro_rules! trace {
    ($($arg:tt)+) => (log::trace!($($arg)+))
}

#[cfg(not(feature = "logger"))]
macro_rules! trace {
    ($($arg:tt)+) => {};
}

pub(crate) use {debug, trace};

#[cfg(feature = "logger")]
pub fn env_logger() -> env_logger::Builder {
    let r_stderr = Box::new(crate::io::r_stderr());
    let target = env_logger::Target::Pipe(r_stderr);
    let mut builder = env_logger::builder();
    builder.target(target);
    builder
}
