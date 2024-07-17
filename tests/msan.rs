#![feature(cfg_sanitize)]

#[cfg(sanitize = "memory")]
use sanitizers::msan;
#[cfg(sanitize = "memory")]
use std::os::raw::c_void;

/// Tests that memory regions can be poisoned and unpoisoned.
#[cfg(sanitize = "memory")]
#[test]
fn msan_basic() {
    let mut data = vec![0u8; 100];
    let data_ptr = data.as_mut_ptr() as *const c_void;

    // Poison the memory region
    msan::poison(data_ptr, data.len());

    // Check if the memory region is poisoned
    let poisoned_offset = msan::test_shadow(data_ptr, data.len());
    assert_ne!(poisoned_offset, -1);

    // Unpoison the memory region
    msan::unpoison(data_ptr, data.len());

    // Check if the memory region is unpoisoned
    let unpoisoned_offset = msan::test_shadow(data_ptr, data.len());
    assert_eq!(unpoisoned_offset, -1);
}
