#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod macro_guard;
pub mod quantum_switch;
pub mod state_vector;
pub mod weak_measurement;

pub use macro_guard::{MacroGuard, MacroScaleBoundaryViolation};
pub use quantum_switch::{BasicQuantumSwitch, Hamiltonian, JointState, Matrix4x4, QuantumSwitch};
pub use state_vector::{BasicStateVector, StateVector};
pub use weak_measurement::{
    BasicWeakMeasurer, Matrix2x2, QubitState, WeakMeasurementError, WeakMeasurer,
};

pub use num_complex::Complex64;
