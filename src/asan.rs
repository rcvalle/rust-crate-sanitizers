/// AddressSanitizer interface.
///
/// For more information about AddressSanitizer, see
/// https://clang.llvm.org/docs/AddressSanitizer.html.
use crate::ffi::asan::*;

use std::ffi::CStr;
use std::os::raw::c_void;

/// Marks a memory region as unaddressable.
pub fn poison_memory_region(addr: *const c_void, size: usize) {
    unsafe {
        __asan_poison_memory_region(addr, size);
    }
}

/// Marks a memory region as addressable.
pub fn unpoison_memory_region(addr: *const c_void, size: usize) {
    unsafe {
        __asan_unpoison_memory_region(addr, size);
    }
}

/// Checks if an address is poisoned.
pub fn is_address_poisoned(addr: *const c_void) -> bool {
    unsafe { __asan_address_is_poisoned(addr) != 0 }
}

/// Checks if a region is poisoned and returns the address of the first poisoned byte.
pub fn region_is_poisoned(beg: *mut c_void, size: usize) -> Option<*mut c_void> {
    let addr = unsafe { __asan_region_is_poisoned(beg, size) };
    if addr.is_null() {
        None
    } else {
        Some(addr)
    }
}

/// Describes an address (useful for calling from the debugger).
pub fn describe_address(addr: *mut c_void) {
    unsafe {
        __asan_describe_address(addr);
    }
}

/// Checks if an error has been or is being reported.
pub fn report_present() -> bool {
    unsafe { __asan_report_present() != 0 }
}

/// Gets the PC (program counter) register value of an ASan error.
pub fn get_report_pc() -> *mut c_void {
    unsafe { __asan_get_report_pc() }
}

/// Gets the BP (base pointer) register value of an ASan error.
pub fn get_report_bp() -> *mut c_void {
    unsafe { __asan_get_report_bp() }
}

/// Gets the SP (stack pointer) register value of an ASan error.
pub fn get_report_sp() -> *mut c_void {
    unsafe { __asan_get_report_sp() }
}

/// Gets the address of the report buffer of an ASan error.
pub fn get_report_address() -> *mut c_void {
    unsafe { __asan_get_report_address() }
}

/// Gets access type of an ASan error (0 = read, 1 = write).
pub fn get_report_access_type() -> i32 {
    unsafe { __asan_get_report_access_type() }
}

/// Gets access size of an ASan error in bytes.
pub fn get_report_access_size() -> usize {
    unsafe { __asan_get_report_access_size() }
}

/// Gets the bug description of an ASan error.
pub fn get_report_description() -> String {
    unsafe {
        let desc_ptr = __asan_get_report_description();
        if desc_ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(desc_ptr).to_string_lossy().into_owned()
        }
    }
}

/// Sets a callback function to be called during ASan error reporting.
pub fn set_error_report_callback(callback: Option<unsafe extern "C" fn(arg1: *const i8)>) {
    unsafe {
        __asan_set_error_report_callback(callback);
    }
}

/// Prints accumulated statistics to stderr.
pub fn print_accumulated_stats() {
    unsafe {
        __asan_print_accumulated_stats();
    }
}

/// User-provided default option settings.
pub fn default_options() -> String {
    unsafe {
        let options_ptr = __asan_default_options();
        if options_ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(options_ptr).to_string_lossy().into_owned()
        }
    }
}
