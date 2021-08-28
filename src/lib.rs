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
#[inline]
fn std_panicking() -> bool {
    false
}

#[cfg(feature="std")]
#[inline]
fn std_panicking() -> bool {
    std::thread::panicking()
}

/// Determines whether the current thread is unwinding because of panic.
///
/// In contrast with
/// [`std::thread::panicking`](https://doc.rust-lang.org/std/thread/fn.panicking.html),
/// this function can be used in the `no_std` context,
/// although the adequacy of the return value in this case is left to the developer
/// of the final application: they supposed to use
/// the [`set_panicking_callback`](set_panicking_callback) function
/// to provide method for unwinding detecting.
pub fn panicking() -> bool {
    let panicking = PANICKING_CALLBACK.load(Ordering::Relaxed);
    let panicking: Option<fn() -> bool> = unsafe { transmute(panicking) };
    if let Some(panicking) = panicking {
        panicking()
    } else {
        std_panicking()
    }
}

pub fn set_panicking_callback(panicking: fn() -> bool) {
    let panicking: Option<fn() -> bool> = Some(panicking);
    let panicking = unsafe { transmute(panicking) };
    PANICKING_CALLBACK.store(panicking, Ordering::Relaxed);
}
