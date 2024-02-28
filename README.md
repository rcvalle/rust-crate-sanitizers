sanitizers
==========

![Build Status](https://github.com/rcvalle/rust-crate-cfi-types/workflows/build/badge.svg)

FFI bindings for the [sanitizers](https://github.com/google/sanitizers)
interfaces.


Installation
------------

To install the `sanitizers` crate:

1. On a command prompt or terminal with your package root's directory as the
   current working directory, run the following command:

       cargo add sanitizers

Or:

1. Add the `sanitizers` crate to your package root's `Cargo.toml` file:

       [dependencies]
       sanitizers = "0.0.1"

2. On a command prompt or terminal with your package root's directory as the
   current working directory, run the following command:

       cargo fetch


Usage
-----

To use the `sanitizers` crate:

1. Import the sanitizer module or funtions from the `sanitizers` crate. E.g.:

       use sanitizers::dfsan::*;

2. Use the provided interface for the sanitizer. E.g.:

       ...
       let mut i = 1i64;
       let i_ptr = &mut i as *mut i64;
       let i_label: dfsan_label = 1;
       unsafe {
           dfsan_set_label(i_label, i_ptr as *mut c_void, size_of::<i64>());
       }

       let new_label = unsafe { dfsan_get_label(i) };
       ...

3. Build your package with the sanitizer enabled. It is recommended to rebuild
   the standard library with the sanitizer enabled by using the Cargo build-std
   feature (i.e., `-Zbuild-std`) when enabling the sanitizer. E.g.:

       RUSTFLAGS="-Clinker=clang -Clink-arg=-fuse-ld=lld -Zsanitizer=dataflow \
         -Zsanitizer-dataflow-abilist=/path/to/abilist.txt" \
         cargo build -Zbuild-std -Zbuild-std-features \
         --target x86_64-unknown-linux


Contributing
------------

See [CONTRIBUTING.md](CONTRIBUTING.md).


License
-------

Licensed under the Apache License, Version 2.0 or the MIT License. See
[LICENSE-APACHE](LICENSE-APACHE) or [LICENSE-MIT](LICENSE-MIT) for license text
and copyright information.
