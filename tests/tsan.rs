#![feature(cfg_sanitize)]

#[cfg(sanitize = "thread")]
use sanitizers::tsan;
#[cfg(sanitize = "thread")]
use std::os::raw::c_void;

/// Tests that memory regions can be locked and unlocked.
#[cfg(sanitize = "thread")]
#[test]
fn tsan_basic() {
    let mut data = vec![0u8; 100];
    let data_ptr = data.as_mut_ptr() as *mut c_void;

    // Acquire a lock
    tsan::acquire(data_ptr);

    // Release the lock
    tsan::release(data_ptr);

    // Check that no mutexes are held
    tsan::check_no_mutexes_held();
}
