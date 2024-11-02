#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cpu;
pub mod gameboy;
pub mod hardware;
mod memory;
mod util;
