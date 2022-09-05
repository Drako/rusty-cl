#![deny(missing_docs)]
#![doc = "Rust(y) binding for OpenCL"]

/// Module containing all the native function declarations.
pub(crate) mod native;

/// Results and error codes.
pub mod result;

/// Primitive types.
pub mod types;

/// Platform related functions and types.
pub mod platform;
