//! **Crate features**

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
mod i {
    use core::panic::UnwindSafe;

    pub type Error = !;

    pub fn error_into_raw(e: Error) -> usize {
        e
    }

    pub unsafe fn error_from_raw(e: usize) -> Error {
        unreachable_unchecked()
    }

    extern "Rust" {
        fn rust_panicking_neither_abort_nor_std_feature_enabled() -> !;
    }

    #[inline]
    pub fn panicking() -> bool {
        unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
    }

    #[inline]
    pub fn catch_unwind<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, Error> {
        unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
    }

    #[inline]
    pub fn resume_unwind(payload: Error) -> ! {
        unsafe { rust_panicking_neither_abort_nor_std_feature_enabled() }
    }
}

#[cfg(all(feature="std", not(feature="abort")))]
mod i {
    use core::panic::UnwindSafe;
    use std::any::Any;

    pub type Error = Box<dyn Any + Send + 'static>;

    pub fn error_into_raw(e: Error) -> usize {
        Box::into_raw(Box::new(e)) as usize
    }

    pub unsafe fn error_from_raw(e: usize) -> Error {
        *Box::from_raw(e as *mut Error)
    }

    #[inline]
    pub fn panicking() -> bool {
        std::thread::panicking()
    }

    #[inline]
    pub fn catch_unwind<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, Error> {
        std::panic::catch_unwind(f)
    }

    #[inline]
    pub fn resume_unwind(payload: Error) -> ! {
        std::panic::resume_unwind(payload)
    }
}

#[cfg(feature="abort")]
mod i {
    use core::panic::UnwindSafe;

    pub type Error = !;

    pub fn error_into_raw(e: Error) -> usize {
        e
    }

    pub unsafe fn error_from_raw(e: usize) -> Error {
        unreachable_unchecked()
    }

    #[inline]
    pub fn panicking() -> bool {
        false
    }

    #[inline]
    pub fn catch_unwind<T, E>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, Error> {
        Ok(f())
    }

    #[inline]
    pub fn resume_unwind(payload: Error) -> ! {
        payload
    }
}

/// Panic payload. This type cannot be explicitly created,
/// and its only purpose is to be returned from [`catch_unwind`],
/// and then passed to [`resume_unwind`].
pub struct Error(i::Error);

impl Error {
    pub fn into_raw(self) -> usize {
        i::error_into_raw(self.0)
    }

    pub unsafe fn from_raw(error: usize) -> Self {
        Error(i::error_from_raw(error))
    }
}

/// Determines whether the current thread is unwinding because of panic.
///
/// In contrast with
/// [`std::thread::panicking`](https://doc.rust-lang.org/std/thread/fn.panicking.html),
/// this function can be used in the `no_std` context.
#[inline]
pub fn panicking() -> bool {
    i::panicking()
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
    i::catch_unwind(f).map_err(Error)
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
    i::resume_unwind(payload.0)
}
