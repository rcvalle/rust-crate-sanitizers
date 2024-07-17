/// DataFlowSanitizer interface.
///
/// For more information about DataFlowSanitizer, see
/// https://clang.llvm.org/docs/DataFlowSanitizer.html.
use crate::ffi::dfsan::*;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};

pub type DfsanLabel = u8;
pub type DfsanOrigin = u32;

/// Computes the union of `l1` and `l2`, resulting in a union label.
pub fn union(l1: DfsanLabel, l2: DfsanLabel) -> DfsanLabel {
    unsafe { dfsan_union(l1, l2) }
}

/// Sets the label for each address in `[addr, addr+size)` to `label`.
pub fn set_label(label: DfsanLabel, addr: *mut c_void, size: usize) {
    unsafe {
        dfsan_set_label(label, addr, size);
    }
}

/// Sets the label for each address in `[addr, addr+size)` to the union of the
/// current label for that address and `label`.
pub fn add_label(label: DfsanLabel, addr: *mut c_void, size: usize) {
    unsafe {
        dfsan_add_label(label, addr, size);
    }
}

/// Retrieves the label associated with the given data.
pub fn get_label(data: c_long) -> DfsanLabel {
    unsafe { dfsan_get_label(data) }
}

/// Retrieves the immediate origin associated with the given data.
pub fn get_origin(data: c_long) -> DfsanOrigin {
    unsafe { dfsan_get_origin(data) }
}

/// Retrieves the label associated with the data at the given address.
pub fn read_label(addr: *const c_void, size: usize) -> DfsanLabel {
    unsafe { dfsan_read_label(addr, size) }
}

/// Return the origin associated with the first taint byte in the size bytes
/// from the address `addr`.
pub fn read_origin_of_first_taint(addr: *const c_void, size: usize) -> DfsanOrigin {
    unsafe { dfsan_read_origin_of_first_taint(addr, size) }
}

/// Returns whether the given label contains the label `elem`.
pub fn has_label(label: DfsanLabel, elem: DfsanLabel) -> bool {
    unsafe { dfsan_has_label(label, elem) != 0 }
}

/// Flushes the DFSan shadow, i.e. forgets about all labels currently associated
/// with the application memory.
pub fn flush() {
    unsafe {
        dfsan_flush();
    }
}

/// Sets a callback to be invoked on calls to write().
pub fn set_write_callback(
    callback: Option<unsafe extern "C" fn(fd: c_int, buf: *const c_void, count: usize)>,
) {
    unsafe {
        dfsan_set_write_callback(callback);
    }
}

/// Sets a callback to be invoked on any conditional expressions which have a
/// taint label set.
pub fn set_conditional_callback(
    callback: Option<unsafe extern "C" fn(label: DfsanLabel, origin: DfsanOrigin)>,
) {
    unsafe {
        dfsan_set_conditional_callback(callback);
    }
}

/// Conditional expressions occur during signal handlers. This function returns
/// all label bits seen in signal handler conditions.
pub fn get_labels_in_signal_conditional() -> DfsanLabel {
    unsafe { dfsan_get_labels_in_signal_conditional() }
}

/// Sets a callback to be invoked when tainted data reaches a function.
pub fn set_reaches_function_callback(
    callback: Option<
        unsafe extern "C" fn(
            label: DfsanLabel,
            origin: DfsanOrigin,
            file: *const c_char,
            line: c_uint,
            function: *const c_char,
        ),
    >,
) {
    unsafe {
        dfsan_set_reaches_function_callback(callback);
    }
}

/// Functions reached in signal handlers will add the labels they see into a
/// global (bitwise-or together). This function returns all label bits seen
/// during signal handlers.
pub fn get_labels_in_signal_reaches_function() -> DfsanLabel {
    unsafe { dfsan_get_labels_in_signal_reaches_function() }
}

/// Prints the origin trace of the label at the address `addr` to stderr.
pub fn print_origin_trace(addr: *const c_void, description: Option<&str>) {
    let description_cstr = description.map(|s| CString::new(s).unwrap());

    unsafe {
        dfsan_print_origin_trace(
            addr,
            description_cstr
                .as_ref()
                .map_or(std::ptr::null(), |s| s.as_ptr()),
        );
    }
}

/// Prints the origin trace of the label at the address `addr` to a
/// pre-allocated output buffer.
pub fn sprint_origin_trace(
    addr: *const c_void,
    description: Option<&str>,
) -> Result<String, String> {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer = vec![0 as c_char; BUFFER_SIZE];
    let description_cstr = description.map(|s| std::ffi::CString::new(s).unwrap());

    unsafe {
        let len = dfsan_sprint_origin_trace(
            addr,
            description_cstr
                .as_ref()
                .map_or(std::ptr::null(), |s| s.as_ptr()),
            buffer.as_mut_ptr(),
            BUFFER_SIZE,
        );

        if len >= BUFFER_SIZE {
            buffer.resize(len as usize + 1, 0);
            dfsan_sprint_origin_trace(
                addr,
                description_cstr
                    .as_ref()
                    .map_or(std::ptr::null(), |s| s.as_ptr()),
                buffer.as_mut_ptr(),
                len as usize + 1,
            );
        }

        Ok(CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .into_owned())
    }
}

/// Prints the stack trace leading to this call to a pre-allocated output
/// buffer.
pub fn sprint_stack_trace() -> Result<String, String> {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer = vec![0 as c_char; BUFFER_SIZE];

    unsafe {
        let len = dfsan_sprint_stack_trace(buffer.as_mut_ptr(), BUFFER_SIZE);

        if len >= BUFFER_SIZE {
            buffer.resize(len as usize + 1, 0);
            dfsan_sprint_stack_trace(buffer.as_mut_ptr(), len as usize + 1);
        }

        Ok(CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .into_owned())
    }
}

/// Retrieves the very first origin associated with the data at the given
/// address.
pub fn get_init_origin(addr: *const c_void) -> DfsanOrigin {
    unsafe { dfsan_get_init_origin(addr) }
}

/// Returns the value of -dfsan-track-origins.
pub fn get_track_origins() -> c_int {
    unsafe { dfsan_get_track_origins() }
}
