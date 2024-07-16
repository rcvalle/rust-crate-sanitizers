#![feature(cfg_sanitize)]

#[cfg(sanitize = "dataflow")]
use sanitizers::dfsan::*;
#[cfg(sanitize = "dataflow")]
use std::mem::size_of;
#[cfg(sanitize = "dataflow")]
use std::os::raw::c_void;

/// Tests that labels are propagated through loads and stores.
#[cfg(sanitize = "dataflow")]
#[test]
fn basic() {
    // Initialize a variable `i` with a value of `1`
    let mut i = 1i64;
    let i_ptr = &mut i as *mut i64;

    // Set a label `i_label` on `i` using `dfsan_set_label`
    let i_label: dfsan_label = 1;
    unsafe {
        dfsan_set_label(i_label, i_ptr as *mut c_void, size_of::<i64>());
    }

    // Read and verify the label using `dfsan_get_label`
    let new_label = unsafe { dfsan_get_label(i) };
    assert_eq!(i_label, new_label);

    // Read and verify the label using `dfsan_read_label`
    let read_label = unsafe { dfsan_read_label(i_ptr as *const c_void, size_of::<i64>()) };
    assert_eq!(i_label, read_label);

    // Add a label `j_label` to `i` using `dfsan_add_label`
    let j_label: dfsan_label = 2;
    unsafe {
        dfsan_add_label(j_label, i_ptr as *mut c_void, size_of::<i64>());
    }

    // Read and verify that `i` has both labels `i_label` and `j_label` using `dfsan_read_label`
    let read_label = unsafe { dfsan_read_label(i_ptr as *const c_void, size_of::<i64>()) };
    assert_eq!(unsafe { dfsan_has_label(read_label, i_label) }, 1);
    assert_eq!(unsafe { dfsan_has_label(read_label, j_label) }, 1);
}
