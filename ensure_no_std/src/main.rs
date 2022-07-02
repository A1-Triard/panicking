#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

mod no_std {
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }
}

use panicking::panicking;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert!(!panicking());
    0
}
