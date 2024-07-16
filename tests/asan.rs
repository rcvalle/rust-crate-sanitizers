#![feature(cfg_sanitize)]

#[cfg(sanitize = "address")]
use sanitizers::asan::*;
#[cfg(sanitize = "address")]
use std::os::raw::c_void;

/// Tests that memory regions can be poisoned and unpoisoned.
#[cfg(sanitize = "address")]
#[test]
fn basic() {
    let mut data = vec![0u8; 100];
    let data_ptr = data.as_mut_ptr() as *const c_void;

    // Poison the memory region
    unsafe {
        __asan_poison_memory_region(data_ptr, data.len());
    }

    // Check if the memory region is poisoned
    let is_poisoned = unsafe { __asan_address_is_poisoned(data_ptr) };
    assert_eq!(is_poisoned, 1);

    // Unpoison the memory region
    unsafe {
        __asan_unpoison_memory_region(data_ptr, data.len());
    }

    // Check if the memory region is unpoisoned
    let is_unpoisoned = unsafe { __asan_address_is_poisoned(data_ptr) };
    assert_eq!(is_unpoisoned, 0);
}
