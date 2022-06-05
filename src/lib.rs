#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

#[doc(hidden)]
pub use core::format_args as std_format_args;

use core::fmt::{self};
use core::mem::transmute;
use core::sync::atomic::{AtomicUsize, Ordering};

static PANICKING_CALLBACK: AtomicUsize = AtomicUsize::new({
    let no_callback: Option<fn() -> bool> = None;
    unsafe { transmute(no_callback) }
});

static PANIC_CALLBACK: AtomicUsize = AtomicUsize::new({
    let no_callback: Option<fn(fmt::Arguments) -> !> = None;
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

#[cfg(not(feature="std"))]
#[inline]
fn std_panic(args: fmt::Arguments) -> ! {
    core::panic!("{}", args)
}

#[cfg(feature="std")]
#[inline]
fn std_panic(args: fmt::Arguments) -> ! {
    std::panic!("{}", args)
}

#[doc(hidden)]
pub fn panic(args: fmt::Arguments) -> ! {
    let panic = PANIC_CALLBACK.load(Ordering::Relaxed);
    let panic: Option<fn(fmt::Arguments) -> !> = unsafe { transmute(panic) };
    if let Some(panic) = panic {
        panic(args)
    } else {
        std_panic(args)
    }
}

/// Panics the current thread.
///
/// By default, calls [`std::panic!`] if the `std` feature is enabled, and [`core::panic!`] if it is not.
/// This behavior can be overridden with [`set_panic_callback`].
#[macro_export]
macro_rules! panic {
    ($fmt:expr $(, $($args:tt)*)?) => { $crate::panic($crate::std_format_args!($fmt $(, $($args)*)?)) };
}

/// Determines whether the current thread is unwinding because of panic.
///
/// In contrast with
/// [`std::thread::panicking`](https://doc.rust-lang.org/std/thread/fn.panicking.html),
/// this function can be used in the `no_std` context,
/// although the adequacy of the return value in this case is left to
/// a final application developer: they are supposed to use
/// the [`set_panicking_callback`] function
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

/// Allows to change the [`panic!`] macro behavior.
pub fn set_panic_callback(panic: fn(fmt::Arguments) -> !) {
    let panic: Option<fn(fmt::Arguments) -> !> = Some(panic);
    let panic = unsafe { transmute(panic) };
    PANIC_CALLBACK.store(panic, Ordering::Relaxed);
}

/// Allows an application that uses the `no_std` environment
/// and custom panic handling system with stack unwinding
/// to inform libraries if the stack is currently unwinding due to panic.
pub fn set_panicking_callback(panicking: fn() -> bool) {
    let panicking: Option<fn() -> bool> = Some(panicking);
    let panicking = unsafe { transmute(panicking) };
    PANICKING_CALLBACK.store(panicking, Ordering::Relaxed);
}
