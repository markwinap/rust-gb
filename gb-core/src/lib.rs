#![cfg_attr(not(feature = "std"), no_std)]
// When using no_std, we need to explicitly bring in alloc
//#![cfg_attr(not(feature = "std"), )]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cpu;
pub mod gameboy;
pub mod hardware;
mod memory;
mod util;
