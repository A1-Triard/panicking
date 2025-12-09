#![deny(warnings)]

#![no_std]
#![no_main]

#[cfg(windows)]
#[link(name="msvcrt")]
extern "C" { }

mod no_std {
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }
}

use core::ffi::{c_char, c_int};
use panicking::panicking;

#[unsafe(no_mangle)]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    assert!(!panicking());
    0
}
