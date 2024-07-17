#![feature(cfg_sanitize)]

#[cfg(sanitize = "thread")]
use sanitizers::tsan::*;
#[cfg(sanitize = "thread")]
use std::os::raw::c_void;

/// Tests that memory regions can be locked and unlocked.
#[cfg(sanitize = "thread")]
#[test]
fn tsan_basic() {
    let mut data = vec![0u8; 100];
    let data_ptr = data.as_mut_ptr() as *mut c_void;

    // Acquire a lock
    unsafe {
        __tsan_acquire(data_ptr);
    }

    // Release the lock
    unsafe {
        __tsan_release(data_ptr);
    }

    // Check that no mutexes are held
    unsafe {
        __tsan_check_no_mutexes_held();
    }
}
