//! **Crate features**
//!
//! * `"std"`
//! Enabled by default. Disable to make the library `#![no_std]`.
//!
//! * `"abort"`
//! Enable to link the library with the (unstable) `panic_abort` standard
//! crate, and make [`panicking`] always return `false`.
//!
//! The crate has two features — `"abort"` and `"std"`, and a final application
//! should enable at least one of them, otherwise a linkage error will be emitted.

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

use core::panic::UnwindSafe;

#[cfg(all(not(feature="abort"), not(feature="std")))]
extern "Rust" {
    fn rust_panicking_neither_abort_nor_std_feature_enabled() -> !;
}

#[cfg(all(not(feature="abort"), not(feature="std")))]
#[inline]
fn panicking_raw() -> bool {
    unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
}

#[cfg(all(not(feature="abort"), not(feature="std")))]
#[inline]
fn catch_unwind_raw<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, RawError> {
    unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
}

#[cfg(all(not(feature="abort"), not(feature="std")))]
#[inline]
fn resume_unwind_raw(payload: RawError) -> ! {
    unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
}

#[cfg(all(feature="std", not(feature="abort")))]
#[inline]
fn panicking_raw() -> bool {
    std::thread::panicking()
}

#[cfg(all(feature="std", not(feature="abort")))]
#[inline]
fn catch_unwind_raw<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, RawError> {
    std::panic::catch_unwind(f)
}

#[cfg(all(feature="std", not(feature="abort")))]
#[inline]
fn resume_unwind_raw(payload: RawError) -> ! {
    std::panic::resume_unwind(payload)
}

#[cfg(feature="abort")]
#[inline]
fn panicking_raw() -> bool {
    false
}

#[cfg(feature="abort")]
#[inline]
fn catch_unwind_raw<T, E>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, RawError> {
    Ok(f())
}

#[cfg(feature="abort")]
#[inline]
fn resume_unwind_raw(payload: RawError) -> ! {
    payload
}

#[cfg(all(feature="std", not(feature="abort")))]
type RawError = std::boxed::Box<dyn std::any::Any + Send + 'static>;

#[cfg(any(not(feature="std"), feature="abort"))]
type RawError = !;

/// Panic payload. This type cannot be explicitly created,
/// and its only purpose is to be returned from [`catch_unwind`],
/// and then passed to [`resume_unwind`].
pub struct Error(RawError);

/// Determines whether the current thread is unwinding because of panic.
///
/// In contrast with
/// [`std::thread::panicking`](https://doc.rust-lang.org/std/thread/fn.panicking.html),
/// this function can be used in the `no_std` context.
#[inline]
pub fn panicking() -> bool {
    panicking_raw()
}

/// Invokes a closure, capturing the cause of an unwinding panic if one occurs.
///
/// This function will return Ok with the closure’s result if
/// the closure does not panic, and will return `Err(_)` if the closure panics.
///
/// If this function returns an `Err`, the error value should be further passed
/// to [`resume_unwind`].
#[inline]
pub fn catch_unwind<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, Error> {
    catch_unwind_raw(f).map_err(Error)
}

/// Triggers a panic without invoking the panic hook.
///
/// This is designed to be used in conjunction with [`catch_unwind`] to,
/// for example, carry a panic across a layer of C code.
///
/// In contrast with
/// [`std::panic::resume_unwind`](https://doc.rust-lang.org/std/panic/fn.resume_unwind.html),
/// this function can be used in the `no_std` context.
#[inline]
pub fn resume_unwind(payload: Error) -> ! {
    resume_unwind_raw(payload.0)
}
