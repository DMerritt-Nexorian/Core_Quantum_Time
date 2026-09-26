//! Core Public Trait Abstractions for Aharonov Framework
//!
//! Provides the primary interfaces for quantum state vectors, weak measurement evaluation,
//! and quantum switch causal order restoration protocols.

use crate::macro_guard::MacroScaleBoundaryViolation;
use crate::quantum_switch::{Hamiltonian, JointState, Matrix4x4};
use crate::weak_measurement::{Matrix2x2, QubitState, WeakMeasurementError};
use num_complex::Complex64;

/// Trait abstracting operations on a quantum state vector.
pub trait StateVector {
    /// Returns the norm squared of the quantum state vector.
    fn norm_sqr(&self) -> f64;

    /// Normalizes the state vector in-place or returns a normalized copy.
    fn normalize(&self) -> Self;

    /// Computes the inner product <self | other>.
    fn inner_product(&self, other: &Self) -> Complex64;
}

/// Trait abstracting weak measurement and negative dwell time calculations.
pub trait WeakMeasurer<S: StateVector> {
    /// Computes the weak value A_w = <psi_f | A | psi_i> / <psi_f | psi_i>.
    fn calculate_weak_value(
        &self,
        initial: &S,
        final_state: &S,
        operator: &Matrix2x2,
    ) -> Result<Complex64, WeakMeasurementError>;

    /// Calculates the weak dwell time inside a resonant medium.
    fn calculate_weak_dwell_time(
        &self,
        initial: &S,
        final_state: &S,
        excited_projector: &Matrix2x2,
        t_0: f64,
    ) -> Result<f64, WeakMeasurementError>;

    /// Validates whether a calculated dwell time is a physically sound weak measurement.
    fn validate_dwell_physicality(&self, initial: &S, final_state: &S, dwell_time: f64) -> bool;
}

/// Trait abstracting indefinite causal order quantum switch and state restoration dynamics.
pub trait QuantumSwitch<S: StateVector> {
    /// Computes time evolution operator U(t) = exp(-i * H * t).
    fn evolve(&self, hamiltonian: &Hamiltonian, duration: f64) -> Matrix2x2;

    /// Computes exact unitary restoration operator R(H, t) = exp(i * H * t).
    fn restore(&self, hamiltonian: &Hamiltonian, duration: f64) -> Matrix2x2;

    /// Constructs the 4x4 Quantum Switch matrix superposing operations U_A and U_B.
    fn build_switch(&self, op_a: &Matrix2x2, op_b: &Matrix2x2) -> Matrix4x4;

    /// Executes state restoration subject to macroscopic boundary safety enforcement.
    fn execute_restoration(
        &self,
        initial_state: &JointState,
        hamiltonian: &Hamiltonian,
        duration: f64,
        particle_count: usize,
    ) -> Result<JointState, MacroScaleBoundaryViolation>;
}

/// Standard unoptimized reference implementation for QubitState vector.
impl StateVector for QubitState {
    fn norm_sqr(&self) -> f64 {
        self.alpha.norm_sqr() + self.beta.norm_sqr()
    }

    fn normalize(&self) -> Self {
        QubitState::normalize(self)
    }

    fn inner_product(&self, other: &Self) -> Complex64 {
        QubitState::inner_product(self, other)
    }
}

/// Standard unoptimized reference implementation for WeakMeasurer.
pub struct ReferenceWeakMeasurer;

impl WeakMeasurer<QubitState> for ReferenceWeakMeasurer {
    fn calculate_weak_value(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        operator: &Matrix2x2,
    ) -> Result<Complex64, WeakMeasurementError> {
        crate::weak_measurement::calculate_weak_value(initial, final_state, operator)
    }

    fn calculate_weak_dwell_time(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        excited_projector: &Matrix2x2,
        t_0: f64,
    ) -> Result<f64, WeakMeasurementError> {
        crate::weak_measurement::calculate_weak_dwell_time(
            initial,
            final_state,
            excited_projector,
            t_0,
        )
    }

    fn validate_dwell_physicality(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        dwell_time: f64,
    ) -> bool {
        crate::weak_measurement::validate_negative_dwell_physicality(
            initial,
            final_state,
            dwell_time,
        )
    }
}

/// Standard unoptimized reference implementation for QuantumSwitch.
pub struct ReferenceQuantumSwitch;

impl QuantumSwitch<QubitState> for ReferenceQuantumSwitch {
    fn evolve(&self, hamiltonian: &Hamiltonian, duration: f64) -> Matrix2x2 {
        hamiltonian.evolve_operator(duration)
    }

    fn restore(&self, hamiltonian: &Hamiltonian, duration: f64) -> Matrix2x2 {
        hamiltonian.restoration_operator(duration)
    }

    fn build_switch(&self, op_a: &Matrix2x2, op_b: &Matrix2x2) -> Matrix4x4 {
        crate::quantum_switch::build_quantum_switch(op_a, op_b)
    }

    fn execute_restoration(
        &self,
        initial_state: &JointState,
        hamiltonian: &Hamiltonian,
        duration: f64,
        particle_count: usize,
    ) -> Result<JointState, MacroScaleBoundaryViolation> {
        let guard = crate::macro_guard::MacroGuard::DEFAULT;
        guard.enforce_bounds(0.0, 0.0, particle_count)?;

        let r_mat = hamiltonian.restoration_operator(duration);
        let u_mat = hamiltonian.evolve_operator(duration);
        let identity = Matrix2x2::IDENTITY;

        let switch_u = crate::quantum_switch::build_quantum_switch(&u_mat, &identity);
        let switch_r = crate::quantum_switch::build_quantum_switch(&r_mat, &identity);

        let evolved = switch_u.apply(initial_state);
        let restored = switch_r.apply(&evolved);

        Ok(restored)
    }
}
