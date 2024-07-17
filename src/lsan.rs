/// LeakSanitizer interface.
///
/// For more information about LeakSanitizer, see
/// https://clang.llvm.org/docs/LeakSanitizer.html.
use crate::ffi::lsan::*;

use std::os::raw::c_void;

/// Disables leak detection.
pub fn disable() {
    unsafe {
        __lsan_disable();
    }
}

/// Enables leak detection.
pub fn enable() {
    unsafe {
        __lsan_enable();
    }
}

/// The heap object into which p points will be treated as a non-leak.
pub fn ignore_object(p: *const c_void) {
    unsafe {
        __lsan_ignore_object(p);
    }
}

/// Memory regions registered through this interface will be treated as sources
/// of live pointers during leak checking.
pub fn register_root_region(p: *const c_void, size: usize) {
    unsafe {
        __lsan_register_root_region(p, size);
    }
}

/// Unregisters a root region previously registered.
pub fn unregister_root_region(p: *const c_void, size: usize) {
    unsafe {
        __lsan_unregister_root_region(p, size);
    }
}

/// Check for leaks now. This function behaves identically to the default
/// end-of-process leak check.
pub fn do_leak_check() {
    unsafe {
        __lsan_do_leak_check();
    }
}

/// Check for leaks now and return whether leaks were found.
pub fn do_recoverable_leak_check() -> bool {
    unsafe { __lsan_do_recoverable_leak_check() != 0 }
}
