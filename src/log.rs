#[cfg(feature = "logger")]
macro_rules! debug {
    ($($arg:tt)+) => (log::debug!($($arg)+))
}

#[cfg(not(feature = "logger"))]
macro_rules! debug {
    ($($arg:tt)+) => {};
}

pub(crate) use debug;

#[cfg(feature = "logger")]
pub fn env_logger() -> env_logger::Builder {
    let r_stdout = Box::new(crate::io::r_stdout());
    let target = env_logger::Target::Pipe(r_stdout);
    let mut builder = env_logger::builder();
    builder.target(target);
    builder
}
