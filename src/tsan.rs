#![allow(non_camel_case_types, non_upper_case_globals)]
/// FFI bindings for the ThreadSanitizer interface.
///
/// For more information about ThreadSanitizer, see
/// https://clang.llvm.org/docs/ThreadSanitizer.html.
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

/// Annotations for custom mutexes.
/// The annotations allow to get better reports (with sets of locked mutexes),
/// detect more types of bugs (e.g. mutex misuses, races between lock/unlock and
/// destruction and potential deadlocks) and improve precision and performance
/// (by ignoring individual atomic operations in mutex code). However, the
/// downside is that annotated mutex code itself is not checked for correctness.
///
/// Mutex creation flags are passed to __tsan_mutex_create annotation.
/// If mutex has no constructor and __tsan_mutex_create is not called,
/// the flags may be passed to __tsan_mutex_pre_lock/__tsan_mutex_post_lock
/// annotations.
///
/// Mutex has static storage duration and no-op constructor and destructor.
/// This effectively makes tsan ignore destroy annotation.
pub const __tsan_mutex_linker_init: c_uint = 1;
/// Mutex is write reentrant.
pub const __tsan_mutex_write_reentrant: c_uint = 2;
/// Mutex is read reentrant.
pub const __tsan_mutex_read_reentrant: c_uint = 4;
/// Mutex does not have static storage duration, and must not be used after
/// its destructor runs.  The opposite of __tsan_mutex_linker_init.
/// If this flag is passed to __tsan_mutex_destroy, then the destruction
/// is ignored unless this flag was previously set on the mutex.
pub const __tsan_mutex_not_static: c_uint = 256;
/// Mutex operation flags:
///
/// Denotes read lock operation.
pub const __tsan_mutex_read_lock: c_uint = 8;
/// Denotes try lock operation.
pub const __tsan_mutex_try_lock: c_uint = 16;
/// Denotes that a try lock operation has failed to acquire the mutex.
pub const __tsan_mutex_try_lock_failed: c_uint = 32;
/// Denotes that the lock operation acquires multiple recursion levels.
/// Number of levels is passed in recursion parameter.
/// This is useful for annotation of e.g. Java builtin monitors,
/// for which wait operation releases all recursive acquisitions of the mutex.
pub const __tsan_mutex_recursive_lock: c_uint = 64;
/// Denotes that the unlock operation releases all recursion levels.
/// Number of released levels is returned and later must be passed to
/// the corresponding __tsan_mutex_post_lock annotation.
pub const __tsan_mutex_recursive_unlock: c_uint = 128;
/// Convenient composed constants.
pub const __tsan_mutex_try_read_lock: c_uint = 24;
pub const __tsan_mutex_try_read_lock_failed: c_uint = 56;
/// Flags for __tsan_switch_to_fiber:
/// Do not establish a happens-before relation between fibers
pub const __tsan_switch_to_fiber_no_sync: c_uint = 1;
extern "C" {
    /// __tsan_release establishes a happens-before relation with a preceding
    /// __tsan_acquire on the same address.
    pub fn __tsan_acquire(addr: *mut c_void);
    pub fn __tsan_release(addr: *mut c_void);
    /// Annotate creation of a mutex.
    /// Supported flags: mutex creation flags.
    pub fn __tsan_mutex_create(addr: *mut c_void, flags: c_uint);
    /// Annotate destruction of a mutex.
    /// Supported flags:
    ///   - __tsan_mutex_linker_init
    ///   - __tsan_mutex_not_static
    pub fn __tsan_mutex_destroy(addr: *mut c_void, flags: c_uint);
    /// Annotate start of lock operation.
    /// Supported flags:
    ///   - __tsan_mutex_read_lock
    ///   - __tsan_mutex_try_lock
    ///   - all mutex creation flags
    pub fn __tsan_mutex_pre_lock(addr: *mut c_void, flags: c_uint);
    /// Annotate end of lock operation.
    /// Supported flags:
    ///   - __tsan_mutex_read_lock (must match __tsan_mutex_pre_lock)
    ///   - __tsan_mutex_try_lock (must match __tsan_mutex_pre_lock)
    ///   - __tsan_mutex_try_lock_failed
    ///   - __tsan_mutex_recursive_lock
    ///   - all mutex creation flags
    pub fn __tsan_mutex_post_lock(
        addr: *mut c_void,
        flags: c_uint,
        recursion: c_int,
    );
    /// Annotate start of unlock operation.
    /// Supported flags:
    ///   - __tsan_mutex_read_lock
    ///   - __tsan_mutex_recursive_unlock
    pub fn __tsan_mutex_pre_unlock(
        addr: *mut c_void,
        flags: c_uint,
    ) -> c_int;
    /// Annotate end of unlock operation.
    /// Supported flags:
    ///   - __tsan_mutex_read_lock (must match __tsan_mutex_pre_unlock)
    pub fn __tsan_mutex_post_unlock(
        addr: *mut c_void,
        flags: c_uint,
    );
    /// Annotate start/end of notify/signal/broadcast operation.
    /// Supported flags: none.
    pub fn __tsan_mutex_pre_signal(
        addr: *mut c_void,
        flags: c_uint,
    );
    pub fn __tsan_mutex_post_signal(
        addr: *mut c_void,
        flags: c_uint,
    );
    /// Annotate start/end of a region of code where lock/unlock/signal operation
    /// diverts to do something else unrelated to the mutex. This can be used to
    /// annotate, for example, calls into cooperative scheduler or contention
    /// profiling code.
    /// These annotations must be called only from within
    /// __tsan_mutex_pre/post_lock, __tsan_mutex_pre/post_unlock,
    /// __tsan_mutex_pre/post_signal regions.
    /// Supported flags: none.
    pub fn __tsan_mutex_pre_divert(
        addr: *mut c_void,
        flags: c_uint,
    );
    pub fn __tsan_mutex_post_divert(
        addr: *mut c_void,
        flags: c_uint,
    );
    /// Check that the current thread does not hold any mutexes,
    /// report a bug report otherwise.
    pub fn __tsan_check_no_mutexes_held();
    /// External race detection API.
    /// Can be used by non-instrumented libraries to detect when their objects are
    /// being used in an unsafe manner.
    ///   - __tsan_external_read/__tsan_external_write annotates the logical reads
    ///       and writes of the object at the specified address. 'caller_pc' should
    ///       be the PC of the library user, which the library can obtain with e.g.
    ///       `__builtin_return_address(0)`.
    ///   - __tsan_external_register_tag registers a 'tag' with the specified name,
    ///       which is later used in read/write annotations to denote the object type
    ///   - __tsan_external_assign_tag can optionally mark a heap object with a tag
    pub fn __tsan_external_register_tag(
        object_type: *const c_char,
    ) -> *mut c_void;
    pub fn __tsan_external_register_header(
        tag: *mut c_void,
        header: *const c_char,
    );
    pub fn __tsan_external_assign_tag(
        addr: *mut c_void,
        tag: *mut c_void,
    );
    pub fn __tsan_external_read(
        addr: *mut c_void,
        caller_pc: *mut c_void,
        tag: *mut c_void,
    );
    pub fn __tsan_external_write(
        addr: *mut c_void,
        caller_pc: *mut c_void,
        tag: *mut c_void,
    );
    /// Fiber switching API.
    ///   - TSAN context for fiber can be created by __tsan_create_fiber
    ///     and freed by __tsan_destroy_fiber.
    ///   - TSAN context of current fiber or thread can be obtained
    ///     by calling __tsan_get_current_fiber.
    ///   - __tsan_switch_to_fiber should be called immediately before switch
    ///     to fiber, such as call of swapcontext.
    ///   - Fiber name can be set by __tsan_set_fiber_name.
    pub fn __tsan_get_current_fiber() -> *mut c_void;
    pub fn __tsan_create_fiber(flags: c_uint) -> *mut c_void;
    pub fn __tsan_destroy_fiber(fiber: *mut c_void);
    pub fn __tsan_switch_to_fiber(
        fiber: *mut c_void,
        flags: c_uint,
    );
    pub fn __tsan_set_fiber_name(
        fiber: *mut c_void,
        name: *const c_char,
    );
    /// User-provided callback invoked on TSan initialization.
    pub fn __tsan_on_initialize();
    /// User-provided callback invoked on TSan shutdown.
    /// `failed` - Nonzero if TSan did detect issues, zero otherwise.
    /// Return `0` if TSan should exit as if no issues were detected.  Return nonzero
    /// if TSan should exit as if issues were detected.
    pub fn __tsan_on_finalize(failed: c_int) -> c_int;
    /// Release TSan internal memory in a best-effort manner.
    pub fn __tsan_flush_memory();
    /// User-provided default TSAN options.
    pub fn __tsan_default_options() -> *const c_char;
    /// User-provided default TSAN suppressions.
    pub fn __tsan_default_suppressions() -> *const c_char;
    /// Returns a report's description.
    ///
    /// Returns a report's description (issue type), number of duplicate issues
    /// found, counts of array data (stack traces, memory operations, locations,
    /// mutexes, threads, unique thread IDs) and a stack trace of a <c>sleep()</c>
    /// call (if one was involved in the issue).
    ///
    /// \param report Opaque pointer to the current report.
    /// \param[out] description Report type description.
    /// \param[out] count Count of duplicate issues.
    /// \param[out] stack_count Count of stack traces.
    /// \param[out] mop_count Count of memory operations.
    /// \param[out] loc_count Count of locations.
    /// \param[out] mutex_count Count of mutexes.
    /// \param[out] thread_count Count of threads.
    /// \param[out] unique_tid_count Count of unique thread IDs.
    /// \param sleep_trace A buffer to store the stack trace of a <c>sleep()</c>
    /// call.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_data(
        report: *mut c_void,
        description: *mut *const c_char,
        count: *mut c_int,
        stack_count: *mut c_int,
        mop_count: *mut c_int,
        loc_count: *mut c_int,
        mutex_count: *mut c_int,
        thread_count: *mut c_int,
        unique_tid_count: *mut c_int,
        sleep_trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about stack traces included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's stacks.
    /// \param trace A buffer to store the stack trace.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_stack(
        report: *mut c_void,
        idx: c_ulong,
        trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about memory operations included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's memory operations.
    /// \param[out] tid Thread ID of the memory operation.
    /// \param[out] addr Address of the memory operation.
    /// \param[out] size Size of the memory operation.
    /// \param[out] write Write flag of the memory operation.
    /// \param[out] atomic Atomicity flag of the memory operation.
    /// \param trace A buffer to store the stack trace.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_mop(
        report: *mut c_void,
        idx: c_ulong,
        tid: *mut c_int,
        addr: *mut *mut c_void,
        size: *mut c_int,
        write: *mut c_int,
        atomic: *mut c_int,
        trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about locations included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's locations.
    /// \param[out] type Type of the location.
    /// \param[out] addr Address of the location.
    /// \param[out] start Start of the location.
    /// \param[out] size Size of the location.
    /// \param[out] tid Thread ID of the location.
    /// \param[out] fd File descriptor of the location.
    /// \param[out] suppressable Suppressable flag.
    /// \param trace A buffer to store the stack trace.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_loc(
        report: *mut c_void,
        idx: c_ulong,
        type_: *mut *const c_char,
        addr: *mut *mut c_void,
        start: *mut *mut c_void,
        size: *mut c_ulong,
        tid: *mut c_int,
        fd: *mut c_int,
        suppressable: *mut c_int,
        trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about mutexes included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's mutexes.
    /// \param[out] mutex_id Id of the mutex.
    /// \param[out] addr Address of the mutex.
    /// \param[out] destroyed Destroyed mutex flag.
    /// \param trace A buffer to store the stack trace.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_mutex(
        report: *mut c_void,
        idx: c_ulong,
        mutex_id: *mut u64,
        addr: *mut *mut c_void,
        destroyed: *mut c_int,
        trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about threads included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's threads.
    /// \param[out] tid Thread ID of the thread.
    /// \param[out] os_id Operating system's ID of the thread.
    /// \param[out] running Running flag of the thread.
    /// \param[out] name Name of the thread.
    /// \param[out] parent_tid ID of the parent thread.
    /// \param trace A buffer to store the stack trace.
    /// \param trace_size Size in bytes of the trace buffer.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_thread(
        report: *mut c_void,
        idx: c_ulong,
        tid: *mut c_int,
        os_id: *mut u64,
        running: *mut c_int,
        name: *mut *const c_char,
        parent_tid: *mut c_int,
        trace: *mut *mut c_void,
        trace_size: c_ulong,
    ) -> c_int;
    /// Returns information about unique thread IDs included in the report.
    ///
    /// \param report Opaque pointer to the current report.
    /// \param idx Index to the report's unique thread IDs.
    /// \param[out] tid Unique thread ID of the report.
    /// \returns Returns 1 if successful, 0 if not.
    pub fn __tsan_get_report_unique_tid(
        report: *mut c_void,
        idx: c_ulong,
        tid: *mut c_int,
    ) -> c_int;
    /// Returns the current report.
    ///
    /// If TSan is currently reporting a detected issue on the current thread,
    /// returns an opaque pointer to the current report. Otherwise returns NULL.
    /// \returns An opaque pointer to the current report. Otherwise returns NULL.
    pub fn __tsan_get_current_report() -> *mut c_void;
}
