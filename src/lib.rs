#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

use core::mem::transmute;
use core::sync::atomic::{AtomicUsize, Ordering};

static PANICKING_CALLBACK: AtomicUsize = AtomicUsize::new({
    let no_callback: Option<fn() -> bool> = None;
    unsafe { transmute(no_callback) }
});

#[cfg(not(feature="std"))]
pub fn panicking() -> bool {
    let panicking = PANICKING_CALLBACK.load(Ordering::Relaxed);
    let panicking: Option<fn() -> bool> = unsafe { transmute(panicking) };
    if let Some(panicking) = panicking {
        panicking()
    } else {
        false
    }
}

#[cfg(feature="std")]
pub fn panicking() -> bool {
    let panicking = PANICKING_CALLBACK.load(Ordering::Relaxed);
    let panicking: Option<fn() -> bool> = unsafe { transmute(panicking) };
    if let Some(panicking) = panicking {
        panicking()
    } else {
        std::thread::panicking()
    }
}

pub fn set_panicking_callback(panicking: fn() -> bool) {
    let panicking: Option<fn() -> bool> = Some(panicking);
    let panicking = unsafe { transmute(panicking) };
    PANICKING_CALLBACK.store(panicking, Ordering::Relaxed);
}
