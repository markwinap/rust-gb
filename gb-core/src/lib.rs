#![cfg_attr(not(feature = "std"), no_std)]

use std::sync::{Arc, LazyLock, Mutex};

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cpu;
pub mod gameboy;
pub mod hardware;
mod memory;
mod util;

static mut ENABLE_LOG: LazyLock<Arc<Mutex<bool>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(false)));

pub fn is_log_enabled() -> bool {
    let enable_log = unsafe { crate::ENABLE_LOG.lock().unwrap() };
    *enable_log
}
