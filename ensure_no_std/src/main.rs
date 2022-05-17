#![feature(default_alloc_error_handler)]
#![feature(start)]

#![deny(warnings)]

#![no_std]

use core::alloc::Layout;
use core::panic::PanicInfo;
#[cfg(not(windows))]
use libc::exit;
use libc_alloc::LibcAlloc;
use panicking::{panicking, set_panicking_callback};
#[cfg(windows)]
use winapi::shared::minwindef::UINT;
#[cfg(windows)]
use winapi::um::processthreadsapi::ExitProcess;

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[cfg(windows)]
unsafe fn exit(code: UINT) -> ! {
    ExitProcess(code);
    loop { }
}

#[panic_handler]
pub extern fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(99) }
}

#[no_mangle]
pub fn rust_oom(_layout: Layout) -> ! {
    unsafe { exit(98) }
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    set_panicking_callback(|| false);
    assert!(!panicking());
    0
}
