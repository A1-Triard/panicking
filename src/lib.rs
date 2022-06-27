//! **Crate features**
//!
//! * `"std"`
//! Enabled by default. Disable to make the library `#![no_std]`.
//!
//! * `"abort"`
//! Enable to link the library with the (unstable) `panic_abort` standard
//! crate, and make [`panicking`] always return `false`.

#![cfg_attr(feature="abort", feature(panic_abort))]

#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

#[cfg(feature="abort")]
extern crate panic_abort;

#[cfg(all(not(feature="abort"), not(feature="std")))]
extern {
    fn rust_panicking() -> bool;
}

#[cfg(all(not(feature="abort"), not(feature="std")))]
#[inline]
fn panicking_raw() -> bool {
    unsafe { rust_panicking() }
}

#[cfg(all(feature="std", not(feature="abort")))]
#[inline]
fn panicking_raw() -> bool {
    std::thread::panicking()
}

#[cfg(feature="abort")]
#[inline]
fn panicking_raw() -> bool {
    false
}

/// Determines whether the current thread is unwinding because of panic.
///
/// In contrast with
/// [`std::thread::panicking`](https://doc.rust-lang.org/std/thread/fn.panicking.html),
/// this function can be used in the `no_std` context,
/// although the adequacy of the return value in this case is left to
/// a final application developer: they are supposed to provide
/// the `extern fn rust_panicking() -> bool` function
/// for unwinding detecting.
#[inline]
pub fn panicking() -> bool {
    panicking_raw()
}
