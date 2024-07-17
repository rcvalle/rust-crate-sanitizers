#![feature(cfg_sanitize)]

#[cfg(sanitize = "address")]
use sanitizers::asan;
#[cfg(sanitize = "address")]
use std::os::raw::c_void;

/// Tests that memory regions can be poisoned and unpoisoned.
#[cfg(sanitize = "address")]
#[test]
fn basic() {
    let mut data = vec![0u8; 100];
    let data_ptr = data.as_mut_ptr() as *const c_void;

    // Poison the memory region
    asan::poison_memory_region(data_ptr, data.len());

    // Check if the memory region is poisoned
    let is_poisoned = asan::is_address_poisoned(data_ptr);
    assert_eq!(is_poisoned, true);

    // Unpoison the memory region
    asan::unpoison_memory_region(data_ptr, data.len());

    // Check if the memory region is unpoisoned
    let is_poisoned = asan::is_address_poisoned(data_ptr);
    assert_eq!(is_poisoned, false);
}
