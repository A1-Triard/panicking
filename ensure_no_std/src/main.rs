#![feature(default_alloc_error_handler)]
#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

mod no_std {
    use core::alloc::Layout;
    use core::panic::PanicInfo;
    #[cfg(not(windows))]
    use libc::exit;
    use libc_alloc::LibcAlloc;
    #[cfg(windows)]
    use winapi::shared::minwindef::UINT;
    #[cfg(windows)]
    use winapi::um::processthreadsapi::ExitProcess;

    #[global_allocator]
    static ALLOCATOR: LibcAlloc = LibcAlloc;

    #[cfg(windows)]
    unsafe fn exit(code: UINT) -> ! {
        ExitProcess(code);
        loop { }
    }

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        unsafe { exit(99) }
    }

    #[no_mangle]
    extern "Rust" fn rust_oom(_layout: Layout) -> ! {
        unsafe { exit(98) }
    }

    #[no_mangle]
    extern "Rust" fn rust_panicking() -> bool {
        false
    }
}

use panicking::panicking;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert!(!panicking());
    0
}
