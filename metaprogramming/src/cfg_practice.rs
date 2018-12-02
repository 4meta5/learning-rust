/// Noticed the frequent use in the parity-codec codebase of #![cfg_attr(condition, attribute)]
/// [relevant article](https://chrismorgan.info/blog/rust-cfg_attr.html)

/// #[cfg(condition)] -- compile the thing it decorates if the condition is true
/// #[cfg_attr(condition, attribute)] -- add the attribute to the thing it decorates if the condition is true

// example:
// want the `#![feature(core, std_misc)]` to exist if the `nightly` Cargo feature is enabled
#![cfg_attr(feature = "nightly", feature(core, std_misc))]

// in parity-codec/src/lib.rs
// if feature is not standard, then no_std and feature(alloc)
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]