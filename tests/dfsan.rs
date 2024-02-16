use sanitizers::dfsan::*;
use std::mem::size_of;
use std::os::raw::c_void;

/// Tests that labels are propagated through loads and stores.
#[test]
pub fn basic() {
    let mut i = 1i64;
    let i_ptr = &mut i as *mut i64;
    let i_label: dfsan_label = 1;
    unsafe {
        dfsan_set_label(i_label, i_ptr as *mut c_void, size_of::<i64>());
    }

    let new_label = unsafe { dfsan_get_label(i) };
    assert_eq!(i_label, new_label);

    let read_label = unsafe { dfsan_read_label(i_ptr as *const c_void, size_of::<i64>()) };
    assert_eq!(i_label, read_label);

    let j_label: dfsan_label = 2;
    unsafe {
        dfsan_add_label(j_label, i_ptr as *mut c_void, size_of::<i64>());
    }

    let read_label = unsafe { dfsan_read_label(i_ptr as *const c_void, size_of::<i64>()) };
    assert_eq!(unsafe { dfsan_has_label(read_label, i_label) }, 1);
    assert_eq!(unsafe { dfsan_has_label(read_label, j_label) }, 1);
}
