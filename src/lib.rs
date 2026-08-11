#![no_std]

// Optional alloc feature for zero-allocation models can still exist, but we stick to pure stack-bounded.
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod macro_guard;
pub mod quantum_switch;
pub mod weak_measurement;

// Re-export common linear algebra helpers or types if helpful.
pub use num_complex::Complex64;
