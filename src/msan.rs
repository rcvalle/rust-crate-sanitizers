#![allow(non_camel_case_types)]
/// FFI bindings for the MemorySanitizer interface.
///
/// For more information about MemorySanitizer, see
/// https://clang.llvm.org/docs/MemorySanitizer.html.
use std::option::Option;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    /// Set raw origin for the memory range.
    pub fn __msan_set_origin(a: *const c_void, size: usize, origin: u32);
    /// Get raw origin for an address.
    pub fn __msan_get_origin(a: *const c_void) -> u32;
    /// Test that this_id is a descendant of prev_id (or they are simply equal).
    /// "descendant" here means they are part of the same chain, created with
    /// __msan_chain_origin.
    pub fn __msan_origin_is_descendant_or_same(this_id: u32, prev_id: u32) -> c_int;
    /// Returns non-zero if tracking origins.
    pub fn __msan_get_track_origins() -> c_int;
    /// Returns the origin id of the latest UMR in the calling thread.
    pub fn __msan_get_umr_origin() -> u32;
    /// Make memory region fully initialized (without changing its contents).
    pub fn __msan_unpoison(a: *const c_void, size: usize);
    /// Make a null-terminated string fully initialized (without changing its
    /// contents).
    pub fn __msan_unpoison_string(a: *const c_char);
    /// Make first n parameters of the next function call fully initialized.
    pub fn __msan_unpoison_param(n: usize);
    /// Make memory region fully uninitialized (without changing its contents).
    /// This is a legacy interface that does not update origin information. Use
    /// __msan_allocated_memory() instead.
    pub fn __msan_poison(a: *const c_void, size: usize);
    /// Make memory region partially uninitialized (without changing its contents).
    pub fn __msan_partial_poison(data: *const c_void, shadow: *mut c_void, size: usize);
    /// Returns the offset of the first (at least partially) poisoned byte in the
    /// memory range, or -1 if the whole range is good.
    pub fn __msan_test_shadow(x: *const c_void, size: usize) -> isize;
    /// Checks that memory range is fully initialized, and reports an error if it
    /// is not.
    pub fn __msan_check_mem_is_initialized(x: *const c_void, size: usize);
    /// For testing:
    /// __msan_set_expect_umr(1);
    /// ... some buggy code ...
    /// __msan_set_expect_umr(0);
    /// The last line will verify that a UMR happened.
    pub fn __msan_set_expect_umr(expect_umr: c_int);
    /// Change the value of keep_going flag. Non-zero value means don't terminate
    /// program execution when an error is detected. This will not affect error in
    /// modules that were compiled without the corresponding compiler flag.
    pub fn __msan_set_keep_going(keep_going: c_int);
    /// Print shadow and origin for the memory range to stderr in a human-readable
    /// format.
    pub fn __msan_print_shadow(x: *const c_void, size: usize);
    /// Print shadow for the memory range to stderr in a minimalistic
    /// human-readable format.
    pub fn __msan_dump_shadow(x: *const c_void, size: usize);
    /// Returns true if running under a dynamic tool (DynamoRio-based).
    pub fn __msan_has_dynamic_component() -> c_int;
    /// Tell MSan about newly allocated memory (ex.: custom allocator).
    /// Memory will be marked uninitialized, with origin at the call site.
    pub fn __msan_allocated_memory(data: *const c_void, size: usize);
    /// Tell MSan about newly destroyed memory. Mark memory as uninitialized.
    pub fn __sanitizer_dtor_callback(data: *const c_void, size: usize);
    pub fn __sanitizer_dtor_callback_fields(data: *const c_void, size: usize);
    pub fn __sanitizer_dtor_callback_vptr(data: *const c_void);
    /// This function may be optionally provided by user and should return
    /// a string containing Msan runtime options. See msan_flags.h for details.
    pub fn __msan_default_options() -> *const c_char;
    /// Deprecated. Call __sanitizer_set_death_callback instead.
    pub fn __msan_set_death_callback(callback: Option<unsafe extern "C" fn()>);
    /// Update shadow for the application copy of size bytes from src to dst.
    /// Src and dst are application addresses. This function does not copy the
    /// actual application memory, it only updates shadow and origin for such
    /// copy. Source and destination regions can overlap.
    pub fn __msan_copy_shadow(dst: *const c_void, src: *const c_void, size: usize);
    /// Disables uninitialized memory checks in interceptors.
    pub fn __msan_scoped_disable_interceptor_checks();
    /// Re-enables uninitialized memory checks in interceptors after a previous
    /// call to __msan_scoped_disable_interceptor_checks.
    pub fn __msan_scoped_enable_interceptor_checks();
    pub fn __msan_start_switch_fiber(bottom: *const c_void, size: usize);
    pub fn __msan_finish_switch_fiber(bottom_old: *mut *const c_void, size_old: *mut usize);
}
