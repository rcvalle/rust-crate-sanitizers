/// FFI bindings for the AddressSanitizer interface.
///
/// For more information about AddressSanitizer, see
/// https://clang.llvm.org/docs/AddressSanitizer.html.
use std::option::Option;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    /// Marks a memory region (<c>[addr, addr+size)</c>) as unaddressable.
    ///
    /// This memory must be previously allocated by your program. Instrumented
    /// code is forbidden from accessing addresses in this region until it is
    /// unpoisoned. This function is not guaranteed to poison the entire region -
    /// it could poison only a subregion of <c>[addr, addr+size)</c> due to ASan
    /// alignment restrictions.
    ///
    /// \note This function is not thread-safe because no two threads can poison or
    /// unpoison memory in the same memory region simultaneously.
    ///
    /// \param addr Start of memory region.
    /// \param size Size of memory region.
    pub fn __asan_poison_memory_region(addr: *const c_void, size: usize);
    /// Marks a memory region (<c>[addr, addr+size)</c>) as addressable.
    ///
    /// This memory must be previously allocated by your program. Accessing
    /// addresses in this region is allowed until this region is poisoned again.
    /// This function could unpoison a super-region of <c>[addr, addr+size)</c> due
    /// to ASan alignment restrictions.
    ///
    /// \note This function is not thread-safe because no two threads can
    /// poison or unpoison memory in the same memory region simultaneously.
    ///
    /// \param addr Start of memory region.
    /// \param size Size of memory region.
    pub fn __asan_unpoison_memory_region(addr: *const c_void, size: usize);
    /// Checks if an address is poisoned.
    ///
    /// Returns 1 if <c><i>addr</i></c> is poisoned (that is, 1-byte read/write
    /// access to this address would result in an error report from ASan).
    /// Otherwise returns 0.
    ///
    /// \param addr Address to check.
    ///
    /// \retval 1 Address is poisoned.
    /// \retval 0 Address is not poisoned.
    pub fn __asan_address_is_poisoned(addr: *const c_void) -> c_int;
    /// Checks if a region is poisoned.
    ///
    /// If at least one byte in <c>[beg, beg+size)</c> is poisoned, returns the
    /// address of the first such byte. Otherwise returns 0.
    ///
    /// \param beg Start of memory region.
    /// \param size Start of memory region.
    /// \returns Address of first poisoned byte.
    pub fn __asan_region_is_poisoned(beg: *mut c_void, size: usize) -> *mut c_void;
    /// Describes an address (useful for calling from the debugger).
    ///
    /// Prints the description of <c><i>addr</i></c>.
    ///
    /// \param addr Address to describe.
    pub fn __asan_describe_address(addr: *mut c_void);
    /// Checks if an error has been or is being reported (useful for calling from
    /// the debugger to get information about an ASan error).
    ///
    /// Returns 1 if an error has been (or is being) reported. Otherwise returns 0.
    ///
    /// \returns 1 if an error has been (or is being) reported. Otherwise returns
    /// 0.
    pub fn __asan_report_present() -> c_int;
    /// Gets the PC (program counter) register value of an ASan error (useful for
    /// calling from the debugger).
    ///
    /// Returns PC if an error has been (or is being) reported.
    /// Otherwise returns 0.
    ///
    /// \returns PC value.
    pub fn __asan_get_report_pc() -> *mut c_void;
    /// Gets the BP (base pointer) register value of an ASan error (useful for
    /// calling from the debugger).
    ///
    /// Returns BP if an error has been (or is being) reported.
    /// Otherwise returns 0.
    ///
    /// \returns BP value.
    pub fn __asan_get_report_bp() -> *mut c_void;
    /// Gets the SP (stack pointer) register value of an ASan error (useful for
    /// calling from the debugger).
    ///
    /// If an error has been (or is being) reported, returns SP.
    /// Otherwise returns 0.
    ///
    /// \returns SP value.
    pub fn __asan_get_report_sp() -> *mut c_void;
    /// Gets the address of the report buffer of an ASan error (useful for calling
    /// from the debugger).
    ///
    /// Returns the address of the report buffer if an error has been (or is being)
    /// reported. Otherwise returns 0.
    ///
    /// \returns Address of report buffer.
    pub fn __asan_get_report_address() -> *mut c_void;
    /// Gets access type of an ASan error (useful for calling from the debugger).
    ///
    /// Returns access type (read or write) if an error has been (or is being)
    /// reported. Otherwise returns 0.
    ///
    /// \returns Access type (0 = read, 1 = write).
    pub fn __asan_get_report_access_type() -> c_int;
    /// Gets access size of an ASan error (useful for calling from the debugger).
    ///
    /// Returns access size if an error has been (or is being) reported. Otherwise
    /// returns 0.
    ///
    /// \returns Access size in bytes.
    pub fn __asan_get_report_access_size() -> usize;
    /// Gets the bug description of an ASan error (useful for calling from a
    /// debugger).
    ///
    /// \returns Returns a bug description if an error has been (or is being)
    /// reported - for example, "heap-use-after-free". Otherwise returns an empty
    /// string.
    pub fn __asan_get_report_description() -> *const c_char;
    /// Gets information about a pointer (useful for calling from the debugger).
    ///
    /// Returns the category of the given pointer as a constant string.
    /// Possible return values are <c>global</c>, <c>stack</c>, <c>stack-fake</c>,
    /// <c>heap</c>, <c>heap-invalid</c>, <c>shadow-low</c>, <c>shadow-gap</c>,
    /// <c>shadow-high</c>, and <c>unknown</c>.
    ///
    /// If the return value is <c>global</c> or <c>stack</c>, tries to also return
    /// the variable name, address, and size. If the return value is <c>heap</c>,
    /// tries to return the chunk address and size. <c><i>name</i></c> should point
    /// to an allocated buffer of size <c><i>name_size</i></c>.
    ///
    /// \param addr Address to locate.
    /// \param name Buffer to store the variable's name.
    /// \param name_size Size in bytes of the variable's name buffer.
    /// \param[out] region_address Address of the region.
    /// \param[out] region_size Size of the region in bytes.
    ///
    /// \returns Returns the category of the given pointer as a constant string.
    pub fn __asan_locate_address(
        addr: *mut c_void,
        name: *mut c_char,
        name_size: usize,
        region_address: *mut *mut c_void,
        region_size: *mut usize,
    ) -> *const c_char;
    /// Gets the allocation stack trace and thread ID for a heap address (useful
    /// for calling from the debugger).
    ///
    /// Stores up to <c><i>size</i></c> frames in <c><i>trace</i></c>. Returns
    /// the number of stored frames or 0 on error.
    ///
    /// \param addr A heap address.
    /// \param trace A buffer to store the stack trace.
    /// \param size Size in bytes of the trace buffer.
    /// \param[out] thread_id The thread ID of the address.
    ///
    /// \returns Returns the number of stored frames or 0 on error.
    pub fn __asan_get_alloc_stack(
        addr: *mut c_void,
        trace: *mut *mut c_void,
        size: usize,
        thread_id: *mut c_int,
    ) -> usize;
    /// Gets the free stack trace and thread ID for a heap address (useful for
    /// calling from the debugger).
    ///
    /// Stores up to <c><i>size</i></c> frames in <c><i>trace</i></c>. Returns
    /// the number of stored frames or 0 on error.
    ///
    /// \param addr A heap address.
    /// \param trace A buffer to store the stack trace.
    /// \param size Size in bytes of the trace buffer.
    /// \param[out] thread_id The thread ID of the address.
    ///
    /// \returns Returns the number of stored frames or 0 on error.
    pub fn __asan_get_free_stack(
        addr: *mut c_void,
        trace: *mut *mut c_void,
        size: usize,
        thread_id: *mut c_int,
    ) -> usize;
    /// Gets the current shadow memory mapping (useful for calling from the
    /// debugger).
    ///
    /// \param[out] shadow_scale Shadow scale value.
    /// \param[out] shadow_offset Offset value.
    pub fn __asan_get_shadow_mapping(shadow_scale: *mut usize, shadow_offset: *mut usize);
    /// This is an internal function that is called to report an error. However,
    /// it is still a part of the interface because you might want to set a
    /// breakpoint on this function in the debugger.
    ///
    /// \param pc <c><i>pc</i></c> value of the ASan error.
    /// \param bp <c><i>bp</i></c> value of the ASan error.
    /// \param sp <c><i>sp</i></c> value of the ASan error.
    /// \param addr Address of the ASan error.
    /// \param is_write True if the error is a write error; false otherwise.
    /// \param access_size Size of the memory access of the ASan error.
    pub fn __asan_report_error(
        pc: *mut c_void,
        bp: *mut c_void,
        sp: *mut c_void,
        addr: *mut c_void,
        is_write: c_int,
        access_size: usize,
    );
    pub fn __asan_set_death_callback(callback: Option<unsafe extern "C" fn()>);
    /// Sets the callback function to be called during ASan error reporting.
    ///
    /// The callback provides a string pointer to the report.
    ///
    /// \param callback User-provided function.
    pub fn __asan_set_error_report_callback(
        callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
    /// User-provided callback on ASan errors.
    ///
    /// You can provide a function that would be called immediately when ASan
    /// detects an error. This is useful in cases when ASan detects an error but
    /// your program crashes before the ASan report is printed.
    pub fn __asan_on_error();
    /// Prints accumulated statistics to <c>stderr</c> (useful for calling from the
    /// debugger).
    pub fn __asan_print_accumulated_stats();
    /// User-provided default option settings.
    ///
    /// You can provide your own implementation of this function to return a string
    /// containing ASan runtime options (for example,
    /// <c>verbosity=1:halt_on_error=0</c>).
    ///
    /// \returns Default options string.
    pub fn __asan_default_options() -> *const c_char;
    /// Gets an opaque handler to the current thread's fake stack.
    ///
    /// Returns an opaque handler to be used by
    /// <c>__asan_addr_is_in_fake_stack()</c>. Returns NULL if the current thread
    /// does not have a fake stack.
    ///
    /// \returns An opaque handler to the fake stack or NULL.
    pub fn __asan_get_current_fake_stack() -> *mut c_void;
    /// Checks if an address belongs to a given fake stack.
    ///
    /// If <c><i>fake_stack</i></c> is non-NULL and <c><i>addr</i></c> belongs to a
    /// fake frame in <c><i>fake_stack</i></c>, returns the address of the real
    /// stack that corresponds to the fake frame and sets <c><i>beg</i></c> and
    /// <c><i>end</i></c> to the boundaries of this fake frame. Otherwise returns
    /// NULL and does not touch <c><i>beg</i></c> and <c><i>end</i></c>.
    ///
    /// If <c><i>beg</i></c> or <c><i>end</i></c> are NULL, they are not touched.
    ///
    /// \note This function can be called from a thread other than the owner of
    /// <c><i>fake_stack</i></c>, but the owner thread needs to be alive.
    ///
    /// \param fake_stack An opaque handler to a fake stack.
    /// \param addr Address to test.
    /// \param[out] beg Beginning of fake frame.
    /// \param[out] end End of fake frame.
    /// \returns Stack address or NULL.
    pub fn __asan_addr_is_in_fake_stack(
        fake_stack: *mut c_void,
        addr: *mut c_void,
        beg: *mut *mut c_void,
        end: *mut *mut c_void,
    ) -> *mut c_void;
    /// Performs shadow memory cleanup of the current thread's stack before a
    /// function marked with the <c>[[noreturn]]</c> attribute is called.
    ///
    /// To avoid false positives on the stack, must be called before no-return
    /// functions like <c>_exit()</c> and <c>execl()</c>.
    pub fn __asan_handle_no_return();
    /// Update allocation stack trace for the given allocation to the current stack
    /// trace. Returns 1 if successful, 0 if not.
    pub fn __asan_update_allocation_context(addr: *mut c_void) -> c_int;
}
