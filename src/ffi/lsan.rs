/// FFI bindings for the LeakSanitizer interface.
///
/// For more information about LeakSanitizer, see
/// https://clang.llvm.org/docs/LeakSanitizer.html.
use std::os::raw::{c_int, c_void};

extern "C" {
    /// Allocations made between calls to __lsan_disable() and __lsan_enable() will
    /// be treated as non-leaks. Disable/enable pairs may be nested.
    pub fn __lsan_disable();
    pub fn __lsan_enable();
    /// The heap object into which p points will be treated as a non-leak.
    pub fn __lsan_ignore_object(p: *const c_void);
    /// Memory regions registered through this interface will be treated as sources
    /// of live pointers during leak checking. Useful if you store pointers in
    /// mapped memory.
    /// Points of note:
    /// - __lsan_unregister_root_region() must be called with the same pointer and
    /// size that have earlier been passed to __lsan_register_root_region()
    /// - LSan will skip any inaccessible memory when scanning a root region. E.g.,
    /// if you map memory within a larger region that you have mprotect'ed, you can
    /// register the entire large region.
    /// - the implementation is not optimized for performance. This interface is
    /// intended to be used for a small number of relatively static regions.
    pub fn __lsan_register_root_region(p: *const c_void, size: usize);
    pub fn __lsan_unregister_root_region(p: *const c_void, size: usize);
    /// Check for leaks now. This function behaves identically to the default
    /// end-of-process leak check. In particular, it will terminate the process if
    /// leaks are found and the exitcode runtime flag is non-zero.
    /// Subsequent calls to this function will have no effect and end-of-process
    /// leak check will not run. Effectively, end-of-process leak check is moved to
    /// the time of first invocation of this function.
    /// By calling this function early during process shutdown, you can instruct
    /// LSan to ignore shutdown-only leaks which happen later on.
    pub fn __lsan_do_leak_check();
    /// Check for leaks now. Returns zero if no leaks have been found or if leak
    /// detection is disabled, non-zero otherwise.
    /// This function may be called repeatedly, e.g. to periodically check a
    /// long-running process. It prints a leak report if appropriate, but does not
    /// terminate the process. It does not affect the behavior of
    /// __lsan_do_leak_check() or the end-of-process leak check, and is not
    /// affected by them.
    pub fn __lsan_do_recoverable_leak_check() -> c_int;
}
