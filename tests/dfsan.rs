#![feature(cfg_sanitize)]

#[cfg(sanitize = "dataflow")]
use sanitizers::dfsan;
#[cfg(sanitize = "dataflow")]
use sanitizers::dfsan::DfsanLabel;
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

    // Set a label `i_label` on `i` using `dfsan::set_label`
    let i_label: DfsanLabel = 1;
    dfsan::set_label(i_label, i_ptr as *mut c_void, size_of::<i64>());

    // Read and verify the label using `dfsan::get_label`
    let new_label = dfsan::get_label(i);
    assert_eq!(i_label, new_label);

    // Read and verify the label using `dfsan::read_label`
    let read_label = dfsan::read_label(i_ptr as *const c_void, size_of::<i64>());
    assert_eq!(i_label, read_label);

    // Add a label `j_label` to `i` using `dfsan::add_label`
    let j_label: DfsanLabel = 2;
    dfsan::add_label(j_label, i_ptr as *mut c_void, size_of::<i64>());

    // Read and verify that `i` has both labels `i_label` and `j_label` using
    // `dfsan::read_label`.
    let read_label = dfsan::read_label(i_ptr as *const c_void, size_of::<i64>());
    assert_eq!(dfsan::has_label(read_label, i_label), true);
    assert_eq!(dfsan::has_label(read_label, j_label), true);
}
