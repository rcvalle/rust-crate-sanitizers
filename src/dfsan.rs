#![allow(non_camel_case_types)]
/// FFI bindings for the DataFlowSanitizer interface.
///
/// For more information about DataFlowSanitizer, see
/// https://clang.llvm.org/docs/DataFlowSanitizer.html.
use std::option::Option;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};

pub type dfsan_label = u8;
pub type dfsan_origin = u32;

/// Signature of the callback argument to dfsan_set_write_callback().
pub type dfsan_write_callback_t =
    Option<unsafe extern "C" fn(fd: c_int, buf: *const c_void, count: usize)>;

/// Signature of the callback argument to dfsan_set_conditional_callback().
pub type dfsan_conditional_callback_t =
    Option<unsafe extern "C" fn(label: dfsan_label, origin: dfsan_origin)>;

/// Signature of the callback argument to dfsan_set_reaches_function_callback().
/// The description is intended to hold the name of the variable.
pub type dfsan_reaches_function_callback_t = Option<
    unsafe extern "C" fn(
        label: dfsan_label,
        origin: dfsan_origin,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ),
>;

extern "C" {
    /// Computes the union of \c l1 and \c l2, resulting in a union label.
    pub fn dfsan_union(l1: dfsan_label, l2: dfsan_label) -> dfsan_label;

    /// Sets the label for each address in [addr,addr+size) to \c label.
    pub fn dfsan_set_label(label: dfsan_label, addr: *mut c_void, size: usize);

    /// Sets the label for each address in [addr,addr+size) to the union of the
    /// current label for that address and \c label.
    pub fn dfsan_add_label(label: dfsan_label, addr: *mut c_void, size: usize);

    /// Retrieves the label associated with the given data.
    ///
    /// The type of 'data' is arbitrary.  The function accepts a value of any type,
    /// which can be truncated or extended (implicitly or explicitly) as necessary.
    /// The truncation/extension operations will preserve the label of the original
    /// value.
    pub fn dfsan_get_label(data: c_long) -> dfsan_label;

    /// Retrieves the immediate origin associated with the given data. The returned
    /// origin may point to another origin.
    ///
    /// The type of 'data' is arbitrary.
    pub fn dfsan_get_origin(data: c_long) -> dfsan_origin;

    /// Retrieves the label associated with the data at the given address.
    pub fn dfsan_read_label(addr: *const c_void, size: usize) -> dfsan_label;

    /// Return the origin associated with the first taint byte in the size bytes
    /// from the address addr.
    pub fn dfsan_read_origin_of_first_taint(addr: *const c_void, size: usize) -> dfsan_origin;

    /// Returns whether the given label contains the label elem.
    pub fn dfsan_has_label(label: dfsan_label, elem: dfsan_label) -> c_int;

    /// Flushes the DFSan shadow, i.e. forgets about all labels currently associated
    /// with the application memory.  Use this call to start over the taint tracking
    /// within the same process.
    ///
    /// Note: If another thread is working with tainted data during the flush, that
    /// taint could still be written to shadow after the flush.
    pub fn dfsan_flush();

    /// Sets a callback to be invoked on calls to write().  The callback is invoked
    /// before the write is done.  The write is not guaranteed to succeed when the
    /// callback executes.  Pass in NULL to remove any callback.
    pub fn dfsan_set_write_callback(labeled_write_callback: dfsan_write_callback_t);

    /// Sets a callback to be invoked on any conditional expressions which have a
    /// taint label set. This can be used to find where tainted data influences
    /// the behavior of the program.
    /// These callbacks will only be added when -dfsan-conditional-callbacks=true.
    pub fn dfsan_set_conditional_callback(callback: dfsan_conditional_callback_t);

    /// Conditional expressions occur during signal handlers.
    /// Making callbacks that handle signals well is tricky, so when
    /// -dfsan-conditional-callbacks=true, conditional expressions used in signal
    /// handlers will add the labels they see into a global (bitwise-or together).
    /// This function returns all label bits seen in signal handler conditions.
    pub fn dfsan_get_labels_in_signal_conditional() -> dfsan_label;

    /// Sets a callback to be invoked when tainted data reaches a function.
    /// This could occur at function entry, or at a load instruction.
    /// These callbacks will only be added if -dfsan-reaches-function-callbacks=1.
    pub fn dfsan_set_reaches_function_callback(callback: dfsan_reaches_function_callback_t);

    /// Making callbacks that handle signals well is tricky, so when
    /// -dfsan-reaches-function-callbacks=true, functions reached in signal
    /// handlers will add the labels they see into a global (bitwise-or together).
    /// This function returns all label bits seen during signal handlers.
    pub fn dfsan_get_labels_in_signal_reaches_function() -> dfsan_label;

    /// Interceptor hooks.
    /// Whenever a dfsan's custom function is called the corresponding
    /// hook is called it non-zero. The hooks should be defined by the user.
    /// The primary use case is taint-guided fuzzing, where the fuzzer
    /// needs to see the parameters of the function and the labels.
    /// FIXME: implement more hooks.
    pub fn dfsan_weak_hook_memcmp(
        caller_pc: *mut c_void,
        s1: *const c_void,
        s2: *const c_void,
        n: usize,
        s1_label: dfsan_label,
        s2_label: dfsan_label,
        n_label: dfsan_label,
    );

    pub fn dfsan_weak_hook_strncmp(
        caller_pc: *mut c_void,
        s1: *const c_char,
        s2: *const c_char,
        n: usize,
        s1_label: dfsan_label,
        s2_label: dfsan_label,
        n_label: dfsan_label,
    );

    /// Prints the origin trace of the label at the address addr to stderr. It also
    /// prints description at the beginning of the trace. If origin tracking is not
    /// on, or the address is not labeled, it prints nothing.
    pub fn dfsan_print_origin_trace(addr: *const c_void, description: *const c_char);

    /// As above, but use an origin id from dfsan_get_origin() instead of address.
    /// Does not include header line with taint label and address information.
    pub fn dfsan_print_origin_id_trace(origin: dfsan_origin);

    /// Prints the origin trace of the label at the address \p addr to a
    /// pre-allocated output buffer. If origin tracking is not on, or the address is
    /// not labeled, it prints nothing.
    ///
    /// Typical usage:
    /// \code
    ///   char kDescription[] = "...";
    ///   char buf\[1024\];
    ///   dfsan_sprint_origin_trace(&tainted_var, kDescription, buf, sizeof(buf));
    /// \endcode
    ///
    /// Typical usage that handles truncation:
    /// \code
    ///   char buf\[1024\];
    ///   int len = dfsan_sprint_origin_trace(&var, nullptr, buf, sizeof(buf));
    ///
    ///   if (len < sizeof(buf)) {
    ///     ProcessOriginTrace(buf);
    ///   } else {
    ///     char *tmpbuf = new char[len + 1];
    ///     dfsan_sprint_origin_trace(&var, nullptr, tmpbuf, len + 1);
    ///     ProcessOriginTrace(tmpbuf);
    ///     delete[] tmpbuf;
    ///   }
    /// \endcode
    ///
    /// \param addr The tainted memory address whose origin we are printing.
    /// \param description A description printed at the beginning of the trace.
    /// \param \[out\] out_buf The output buffer to write the results to.
    /// \param out_buf_size The size of \p out_buf.
    ///
    /// \returns The number of symbols that should have been written to \p out_buf
    /// (not including trailing null byte '\0'). Thus, the string is truncated iff
    /// return value is not less than \p out_buf_size.
    pub fn dfsan_sprint_origin_trace(
        addr: *const c_void,
        description: *const c_char,
        out_buf: *mut c_char,
        out_buf_size: usize,
    ) -> usize;

    /// As above, but use an origin id from dfsan_get_origin() instead of address.
    /// Does not include header line with taint label and address information.
    pub fn dfsan_sprint_origin_id_trace(
        origin: dfsan_origin,
        out_buf: *mut c_char,
        out_buf_size: usize,
    ) -> usize;

    /// Prints the stack trace leading to this call to a pre-allocated output
    /// buffer.
    ///
    /// For usage examples, see dfsan_sprint_origin_trace.
    ///
    /// \param \[out\] out_buf The output buffer to write the results to.
    /// \param out_buf_size The size of \p out_buf.
    ///
    /// \returns The number of symbols that should have been written to \p out_buf
    /// (not including trailing null byte '\0'). Thus, the string is truncated iff
    /// return value is not less than \p out_buf_size.
    pub fn dfsan_sprint_stack_trace(out_buf: *mut c_char, out_buf_size: usize) -> usize;

    /// Retrieves the very first origin associated with the data at the given
    /// address.
    pub fn dfsan_get_init_origin(addr: *const c_void) -> dfsan_origin;

    /// Returns the value of -dfsan-track-origins.
    /// * 0: do not track origins.
    /// * 1: track origins at memory store operations.
    /// * 2: track origins at memory load and store operations.
    pub fn dfsan_get_track_origins() -> c_int;
}
