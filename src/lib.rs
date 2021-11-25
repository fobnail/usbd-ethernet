#![no_std]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "log"))]
#[macro_use]
mod dummy_log;

mod buffer;
mod eem;

pub use eem::EemDriver;
