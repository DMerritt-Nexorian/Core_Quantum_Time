//! # Aharonov Framework
//!
//! Aharonov is a pure `#![no_std]` Rust framework designed for zero-allocation, deterministic
//! quantum state temporal reversal, weak measurement, and negative dwell time simulations.
//!
//! ## License & Usage Notice
//! Strictly restricted to non-commercial academic evaluation and research only.
//! Production, commercial, or enterprise use requires a paid Commercial Production License
//! from **Aharonov Software** (Contact: `licensing@aharonov.io`).

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod macro_guard;
pub mod quantum_switch;
pub mod traits;
pub mod weak_measurement;

// Re-export common types and core traits.
pub use num_complex::Complex64;
pub use traits::{QuantumSwitch, StateVector, WeakMeasurer};
