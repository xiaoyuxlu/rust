//! Unwinding for *uefi* target.
//!
//! Right now we don't support this, so this is just stubs.

use alloc::boxed::Box;
use core::any::Any;
use core::intrinsics;

pub unsafe fn cleanup(_ptr: *mut u8) -> Box<dyn Any + Send> {
    // TBD:
    intrinsics::abort()
}

pub unsafe fn panic(_data: Box<dyn Any + Send>) -> u32 {
    // TBD:
    intrinsics::abort()
}
