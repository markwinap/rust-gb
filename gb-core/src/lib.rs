#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cpu;
pub mod gameboy;
pub mod hardware;
mod memory;
mod util;

#[cfg(feature = "defmt-log")]
use defmt::{debug, trace, warn};

#[cfg(all(not(feature = "defmt-log"),))]
#[macro_export]
/// Like log::debug! but does nothing at all
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(all(not(feature = "defmt-log"),))]
#[macro_export]
/// Like log::trace! but does nothing at all
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(all(not(feature = "defmt-log"),))]
#[macro_export]
/// Like log::warn! but does nothing at all
macro_rules! warn {
    ($($arg:tt)+) => {};
}

// static mut ENABLE_LOG: LazyLock<Arc<Mutex<bool>>> =
//     std::sync::LazyLock::new(|| Arc::new(Mutex::new(false)));

#[inline(always)]
pub fn is_log_enabled() -> bool {
    // let enable_log = unsafe { crate::ENABLE_LOG.lock().unwrap() };
    // *enable_log
    false
}
#[inline(always)]
pub fn enable_logging() {}
