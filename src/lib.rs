#![no_std]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(feature = "smoltcp_integration")]
extern crate smoltcp as smoltcp_crate;

#[cfg(not(feature = "log"))]
#[macro_use]
mod dummy_log;

mod buffer;
mod eem;
#[cfg(feature = "smoltcp_integration")]
mod smoltcp;

#[cfg(feature = "smoltcp_integration")]
pub use crate::smoltcp::*;
pub use eem::EthernetDriver;
