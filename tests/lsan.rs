#![feature(cfg_sanitize)]

#[cfg(sanitize = "leak")]
use sanitizers::lsan;
#[cfg(sanitize = "leak")]
use std::mem::forget;
#[cfg(sanitize = "leak")]
use std::os::raw::c_void;

/// Tests that memory regions can be ignored for leak checking.
#[cfg(sanitize = "leak")]
#[test]
fn basic() {
    (|| {
        let mut data = vec![0u8; 100];
        let data_ptr = data.as_mut_ptr() as *const c_void;

        // Takes ownership of the data and forgets it
        forget(data);

        // Ignore the memory region for leak checking
        lsan::ignore_object(data_ptr);
    })();

    // Ensure no leaks are detected
    let leaks_detected = lsan::do_recoverable_leak_check();
    assert_eq!(leaks_detected, false);
}
