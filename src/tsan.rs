/// ThreadSanitizer interface.
///
/// For more information about ThreadSanitizer, see
/// https://clang.llvm.org/docs/ThreadSanitizer.html.
use crate::ffi::tsan::*;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

/// Struct to hold general report data.
pub struct TsanReportData {
    pub description: String,
    pub count: c_int,
    pub stack_count: c_int,
    pub mop_count: c_int,
    pub loc_count: c_int,
    pub mutex_count: c_int,
    pub thread_count: c_int,
    pub unique_tid_count: c_int,
    pub sleep_trace: Vec<*mut c_void>,
}

/// Struct to hold memory operation report data.
pub struct TsanReportMop {
    pub tid: c_int,
    pub addr: *mut c_void,
    pub size: c_int,
    pub write: c_int,
    pub atomic: c_int,
    pub trace: Vec<*mut c_void>,
}

/// Struct to hold location report data.
pub struct TsanReportLoc {
    pub type_: String,
    pub addr: *mut c_void,
    pub start: *mut c_void,
    pub size: c_ulong,
    pub tid: c_int,
    pub fd: c_int,
    pub suppressable: c_int,
    pub trace: Vec<*mut c_void>,
}

/// Struct to hold mutex report data.
pub struct TsanReportMutex {
    pub mutex_id: u64,
    pub addr: *mut c_void,
    pub destroyed: c_int,
    pub trace: Vec<*mut c_void>,
}

/// Struct to hold thread report data.
pub struct TsanReportThread {
    pub tid: c_int,
    pub os_id: u64,
    pub running: c_int,
    pub name: String,
    pub parent_tid: c_int,
    pub trace: Vec<*mut c_void>,
}

/// Establishes a happens-before relation with a preceding acquire on the same
/// address.
pub fn acquire(addr: *mut c_void) {
    unsafe {
        __tsan_acquire(addr);
    }
}

/// Establishes a happens-before relation with a subsequent release on the same
/// address.
pub fn release(addr: *mut c_void) {
    unsafe {
        __tsan_release(addr);
    }
}

/// Annotate creation of a mutex.
pub fn mutex_create(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_create(addr, flags);
    }
}

/// Annotate destruction of a mutex.
pub fn mutex_destroy(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_destroy(addr, flags);
    }
}

/// Annotate start of lock operation.
pub fn mutex_pre_lock(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_pre_lock(addr, flags);
    }
}

/// Annotate end of lock operation.
pub fn mutex_post_lock(addr: *mut c_void, flags: c_uint, recursion: c_int) {
    unsafe {
        __tsan_mutex_post_lock(addr, flags, recursion);
    }
}

/// Annotate start of unlock operation.
pub fn mutex_pre_unlock(addr: *mut c_void, flags: c_uint) -> c_int {
    unsafe { __tsan_mutex_pre_unlock(addr, flags) }
}

/// Annotate end of unlock operation.
pub fn mutex_post_unlock(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_post_unlock(addr, flags);
    }
}

/// Annotate start of notify operation.
pub fn mutex_pre_signal(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_pre_signal(addr, flags);
    }
}

/// Annotate end of notify operation.
pub fn mutex_post_signal(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_post_signal(addr, flags);
    }
}

/// Annotate a region of code where lock/unlock/signal operation diverts to do
/// something else unrelated to the mutex.
pub fn mutex_pre_divert(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_pre_divert(addr, flags);
    }
}

/// Annotate end of a region of code where lock/unlock/signal operation diverts
/// to do something else unrelated to the mutex.
pub fn mutex_post_divert(addr: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_mutex_post_divert(addr, flags);
    }
}

/// Check that the current thread does not hold any mutexes, report a bug
/// otherwise.
pub fn check_no_mutexes_held() {
    unsafe {
        __tsan_check_no_mutexes_held();
    }
}

/// Registers a tag with the specified name.
pub fn external_register_tag(object_type: &str) -> *mut c_void {
    let object_type_cstr = CString::new(object_type).unwrap();
    unsafe { __tsan_external_register_tag(object_type_cstr.as_ptr()) }
}

/// Registers a header for an external tag.
pub fn external_register_header(tag: *mut c_void, header: &str) {
    let header_cstr = CString::new(header).unwrap();
    unsafe {
        __tsan_external_register_header(tag, header_cstr.as_ptr());
    }
}

/// Assigns a tag to a heap object.
pub fn external_assign_tag(addr: *mut c_void, tag: *mut c_void) {
    unsafe {
        __tsan_external_assign_tag(addr, tag);
    }
}

/// Annotates a logical read of the object at the specified address.
pub fn external_read(addr: *mut c_void, caller_pc: *mut c_void, tag: *mut c_void) {
    unsafe {
        __tsan_external_read(addr, caller_pc, tag);
    }
}

/// Annotates a logical write of the object at the specified address.
pub fn external_write(addr: *mut c_void, caller_pc: *mut c_void, tag: *mut c_void) {
    unsafe {
        __tsan_external_write(addr, caller_pc, tag);
    }
}

/// Creates a fiber.
pub fn create_fiber(flags: c_uint) -> *mut c_void {
    unsafe { __tsan_create_fiber(flags) }
}

/// Destroys a fiber.
pub fn destroy_fiber(fiber: *mut c_void) {
    unsafe {
        __tsan_destroy_fiber(fiber);
    }
}

/// Switches to a fiber.
pub fn switch_to_fiber(fiber: *mut c_void, flags: c_uint) {
    unsafe {
        __tsan_switch_to_fiber(fiber, flags);
    }
}

/// Sets a fiber name.
pub fn set_fiber_name(fiber: *mut c_void, name: &str) {
    let name_cstr = CString::new(name).unwrap();
    unsafe {
        __tsan_set_fiber_name(fiber, name_cstr.as_ptr());
    }
}

/// User-provided callback invoked on TSan initialization.
pub fn on_initialize() {
    unsafe {
        __tsan_on_initialize();
    }
}

/// User-provided callback invoked on TSan shutdown.
pub fn on_finalize(failed: c_int) -> c_int {
    unsafe { __tsan_on_finalize(failed) }
}

/// Release TSan internal memory in a best-effort manner.
pub fn flush_memory() {
    unsafe {
        __tsan_flush_memory();
    }
}

/// Returns a report's description.
pub fn get_report_data(report: *mut c_void) -> TsanReportData {
    unsafe {
        let mut description: *const c_char = std::ptr::null();
        let mut count: c_int = 0;
        let mut stack_count: c_int = 0;
        let mut mop_count: c_int = 0;
        let mut loc_count: c_int = 0;
        let mut mutex_count: c_int = 0;
        let mut thread_count: c_int = 0;
        let mut unique_tid_count: c_int = 0;
        let mut sleep_trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];

        __tsan_get_report_data(
            report,
            &mut description,
            &mut count,
            &mut stack_count,
            &mut mop_count,
            &mut loc_count,
            &mut mutex_count,
            &mut thread_count,
            &mut unique_tid_count,
            sleep_trace.as_mut_ptr(),
            1024,
        );

        TsanReportData {
            description: if description.is_null() {
                String::new()
            } else {
                CStr::from_ptr(description).to_string_lossy().into_owned()
            },
            count,
            stack_count,
            mop_count,
            loc_count,
            mutex_count,
            thread_count,
            unique_tid_count,
            sleep_trace,
        }
    }
}

/// Returns information about stack traces included in the report.
pub fn get_report_stack(report: *mut c_void, idx: c_ulong) -> Vec<*mut c_void> {
    unsafe {
        let mut trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];
        __tsan_get_report_stack(report, idx, trace.as_mut_ptr(), 1024);
        trace
    }
}

/// Returns information about memory operations included in the report.
pub fn get_report_mop(report: *mut c_void, idx: c_ulong) -> TsanReportMop {
    unsafe {
        let mut tid: c_int = 0;
        let mut addr: *mut c_void = std::ptr::null_mut();
        let mut size: c_int = 0;
        let mut write: c_int = 0;
        let mut atomic: c_int = 0;
        let mut trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];

        __tsan_get_report_mop(
            report,
            idx,
            &mut tid,
            &mut addr,
            &mut size,
            &mut write,
            &mut atomic,
            trace.as_mut_ptr(),
            1024,
        );

        TsanReportMop {
            tid,
            addr,
            size,
            write,
            atomic,
            trace,
        }
    }
}

/// Returns information about locations included in the report.
pub fn get_report_loc(report: *mut c_void, idx: c_ulong) -> TsanReportLoc {
    unsafe {
        let mut type_: *const c_char = std::ptr::null();
        let mut addr: *mut c_void = std::ptr::null_mut();
        let mut start: *mut c_void = std::ptr::null_mut();
        let mut size: c_ulong = 0;
        let mut tid: c_int = 0;
        let mut fd: c_int = 0;
        let mut suppressable: c_int = 0;
        let mut trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];

        __tsan_get_report_loc(
            report,
            idx,
            &mut type_,
            &mut addr,
            &mut start,
            &mut size,
            &mut tid,
            &mut fd,
            &mut suppressable,
            trace.as_mut_ptr(),
            1024,
        );

        TsanReportLoc {
            type_: if type_.is_null() {
                String::new()
            } else {
                CStr::from_ptr(type_).to_string_lossy().into_owned()
            },
            addr,
            start,
            size,
            tid,
            fd,
            suppressable,
            trace,
        }
    }
}

/// Returns information about mutexes included in the report.
pub fn get_report_mutex(report: *mut c_void, idx: c_ulong) -> TsanReportMutex {
    unsafe {
        let mut mutex_id: u64 = 0;
        let mut addr: *mut c_void = std::ptr::null_mut();
        let mut destroyed: c_int = 0;
        let mut trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];

        __tsan_get_report_mutex(
            report,
            idx,
            &mut mutex_id,
            &mut addr,
            &mut destroyed,
            trace.as_mut_ptr(),
            1024,
        );

        TsanReportMutex {
            mutex_id,
            addr,
            destroyed,
            trace,
        }
    }
}

/// Returns information about threads included in the report.
pub fn get_report_thread(report: *mut c_void, idx: c_ulong) -> TsanReportThread {
    unsafe {
        let mut tid: c_int = 0;
        let mut os_id: u64 = 0;
        let mut running: c_int = 0;
        let mut name: *const c_char = std::ptr::null();
        let mut parent_tid: c_int = 0;
        let mut trace: Vec<*mut c_void> = vec![std::ptr::null_mut(); 1024];

        __tsan_get_report_thread(
            report,
            idx,
            &mut tid,
            &mut os_id,
            &mut running,
            &mut name,
            &mut parent_tid,
            trace.as_mut_ptr(),
            1024,
        );

        TsanReportThread {
            tid,
            os_id,
            running,
            name: if name.is_null() {
                String::new()
            } else {
                CStr::from_ptr(name).to_string_lossy().into_owned()
            },
            parent_tid,
            trace,
        }
    }
}

/// Returns information about unique thread IDs included in the report.
pub fn get_report_unique_tid(report: *mut c_void, idx: c_ulong) -> c_int {
    unsafe {
        let mut tid: c_int = 0;
        __tsan_get_report_unique_tid(report, idx, &mut tid);
        tid
    }
}

/// Returns the current report.
pub fn get_current_report() -> *mut c_void {
    unsafe { __tsan_get_current_report() }
}
