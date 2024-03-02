#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "fake-libR"))]
mod external;
#[cfg(not(feature = "fake-libR"))]
pub use external::*;

#[cfg(feature = "fake-libR")]
mod mock;
#[cfg(feature = "fake-libR")]
pub use mock::*;
