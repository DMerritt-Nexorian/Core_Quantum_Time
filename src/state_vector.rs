#![allow(clippy::needless_range_loop)]

use crate::quantum_switch::Hamiltonian;
use crate::weak_measurement::{Matrix2x2, QubitState};

/// Strict public trait interface for state vector operations, time evolution, and matrix perturbation shocks.
pub trait StateVector {
    /// Evolve state vector forward in time under given Hamiltonian
    fn evolve_time(&self, state: &QubitState, hamiltonian: &Hamiltonian, dt: f64) -> QubitState;

    /// Apply an arbitrary shock / perturbation matrix to state vector
    fn apply_shock_matrix(&self, state: &QubitState, shock: &Matrix2x2) -> QubitState;
}

/// Basic unoptimized pure-Rust reference implementation of `StateVector`.
/// Proprietary zero-allocation byte arena types remain private.
#[derive(Debug, Clone, Copy, Default)]
pub struct BasicStateVector;

impl BasicStateVector {
    pub fn new() -> Self {
        Self
    }
}

impl StateVector for BasicStateVector {
    fn evolve_time(&self, state: &QubitState, hamiltonian: &Hamiltonian, dt: f64) -> QubitState {
        let u_t = hamiltonian.evolve_operator(dt);
        state.apply_matrix(&u_t)
    }

    fn apply_shock_matrix(&self, state: &QubitState, shock: &Matrix2x2) -> QubitState {
        state.apply_matrix(shock)
    }
}
