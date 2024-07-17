/// MemorySanitizer interface.
///
/// For more information about MemorySanitizer, see
/// https://clang.llvm.org/docs/MemorySanitizer.html.
use crate::ffi::msan::*;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

/// Set raw origin for the memory range.
pub fn set_origin(a: *const c_void, size: usize, origin: u32) {
    unsafe {
        __msan_set_origin(a, size, origin);
    }
}

/// Get raw origin for an address.
pub fn get_origin(a: *const c_void) -> u32 {
    unsafe { __msan_get_origin(a) }
}

/// Test that this_id is a descendant of prev_id (or they are simply equal).
pub fn origin_is_descendant_or_same(this_id: u32, prev_id: u32) -> bool {
    unsafe { __msan_origin_is_descendant_or_same(this_id, prev_id) != 0 }
}

/// Returns non-zero if tracking origins.
pub fn get_track_origins() -> c_int {
    unsafe { __msan_get_track_origins() }
}

/// Returns the origin id of the latest UMR in the calling thread.
pub fn get_umr_origin() -> u32 {
    unsafe { __msan_get_umr_origin() }
}

/// Make memory region fully initialized (without changing its contents).
pub fn unpoison(a: *const c_void, size: usize) {
    unsafe {
        __msan_unpoison(a, size);
    }
}

/// Make a null-terminated string fully initialized (without changing its
/// contents).
pub fn unpoison_string(a: *const c_char) {
    unsafe {
        __msan_unpoison_string(a);
    }
}

/// Make first n parameters of the next function call fully initialized.
pub fn unpoison_param(n: usize) {
    unsafe {
        __msan_unpoison_param(n);
    }
}

/// Make memory region fully uninitialized (without changing its contents).
pub fn poison(a: *const c_void, size: usize) {
    unsafe {
        __msan_poison(a, size);
    }
}

/// Returns the offset of the first (at least partially) poisoned byte in the
/// memory range, or -1 if the whole range is good.
pub fn test_shadow(x: *const c_void, size: usize) -> isize {
    unsafe { __msan_test_shadow(x, size) }
}

/// Checks that memory range is fully initialized, and reports an error if it is
/// not.
pub fn check_mem_is_initialized(x: *const c_void, size: usize) {
    unsafe {
        __msan_check_mem_is_initialized(x, size);
    }
}

/// For testing: Set expected uninitialized memory reads.
pub fn set_expect_umr(expect_umr: c_int) {
    unsafe {
        __msan_set_expect_umr(expect_umr);
    }
}

/// Change the value of keep_going flag.
pub fn set_keep_going(keep_going: c_int) {
    unsafe {
        __msan_set_keep_going(keep_going);
    }
}

/// Print shadow and origin for the memory range to stderr in a human-readable
/// format.
pub fn print_shadow(x: *const c_void, size: usize) {
    unsafe {
        __msan_print_shadow(x, size);
    }
}

/// Print shadow for the memory range to stderr in a minimalistic human-readable
/// format.
pub fn dump_shadow(x: *const c_void, size: usize) {
    unsafe {
        __msan_dump_shadow(x, size);
    }
}

/// Returns true if running under a dynamic tool (DynamoRio-based).
pub fn has_dynamic_component() -> bool {
    unsafe { __msan_has_dynamic_component() != 0 }
}

/// Tell MSan about newly allocated memory (ex.: custom allocator).
pub fn allocated_memory(data: *const c_void, size: usize) {
    unsafe {
        __msan_allocated_memory(data, size);
    }
}

/// Tell MSan about newly destroyed memory. Mark memory as uninitialized.
pub fn dtor_callback(data: *const c_void, size: usize) {
    unsafe {
        __sanitizer_dtor_callback(data, size);
    }
}

/// Tell MSan about newly destroyed memory. Mark memory as uninitialized.
pub fn dtor_callback_fields(data: *const c_void, size: usize) {
    unsafe {
        __sanitizer_dtor_callback_fields(data, size);
    }
}

/// Tell MSan about newly destroyed memory. Mark memory as uninitialized.
pub fn dtor_callback_vptr(data: *const c_void) {
    unsafe {
        __sanitizer_dtor_callback_vptr(data);
    }
}

/// This function may be optionally provided by user and should return a string
/// containing Msan runtime options.
pub fn default_options() -> String {
    unsafe {
        let options_ptr = __msan_default_options();
        if options_ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(options_ptr).to_string_lossy().into_owned()
        }
    }
}

/// Update shadow for the application copy of size bytes from src to dst.
pub fn copy_shadow(dst: *const c_void, src: *const c_void, size: usize) {
    unsafe {
        __msan_copy_shadow(dst, src, size);
    }
}

/// Disables uninitialized memory checks in interceptors.
pub fn scoped_disable_interceptor_checks() {
    unsafe {
        __msan_scoped_disable_interceptor_checks();
    }
}

/// Re-enables uninitialized memory checks in interceptors after a previous call
/// to `scoped_disable_interceptor_checks`.
pub fn scoped_enable_interceptor_checks() {
    unsafe {
        __msan_scoped_enable_interceptor_checks();
    }
}

/// Start a fiber switch.
pub fn start_switch_fiber(bottom: *const c_void, size: usize) {
    unsafe {
        __msan_start_switch_fiber(bottom, size);
    }
}

/// Finish a fiber switch.
pub fn finish_switch_fiber(bottom_old: *mut *const c_void, size_old: *mut usize) {
    unsafe {
        __msan_finish_switch_fiber(bottom_old, size_old);
    }
}
