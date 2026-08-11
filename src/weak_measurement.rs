#![allow(clippy::needless_range_loop)]

use num_complex::Complex64;

/// Represents a stack-bounded 2x2 complex matrix for qubit operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub data: [[Complex64; 2]; 2],
}

impl Matrix2x2 {
    pub const ZERO: Self = Self {
        data: [
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
        ],
    };

    pub const IDENTITY: Self = Self {
        data: [
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        ],
    };

    pub fn new(data: [[Complex64; 2]; 2]) -> Self {
        Self { data }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                res[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Self::new(res)
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                let mut sum = Complex64::new(0.0, 0.0);
                for k in 0..2 {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res[i][j] = sum;
            }
        }
        Self::new(res)
    }

    pub fn scale(&self, scalar: Complex64) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                res[i][j] = self.data[i][j] * scalar;
            }
        }
        Self::new(res)
    }

    pub fn conjugate_transpose(&self) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                res[i][j] = self.data[j][i].conj();
            }
        }
        Self::new(res)
    }

    pub fn trace(&self) -> Complex64 {
        self.data[0][0] + self.data[1][1]
    }
}

/// Represents a 2D complex state vector (qubit).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QubitState {
    pub alpha: Complex64,
    pub beta: Complex64,
}

impl QubitState {
    pub fn new(alpha: Complex64, beta: Complex64) -> Self {
        Self { alpha, beta }
    }

    /// Normalizes the qubit state.
    pub fn normalize(&self) -> Self {
        let norm_sq = self.alpha.norm_sqr() + self.beta.norm_sqr();
        if norm_sq > 1e-15 {
            let norm = libm::sqrt(norm_sq);
            Self {
                alpha: self.alpha / norm,
                beta: self.beta / norm,
            }
        } else {
            *self
        }
    }

    /// Inner product: <self | other>
    pub fn inner_product(&self, other: &Self) -> Complex64 {
        self.alpha.conj() * other.alpha + self.beta.conj() * other.beta
    }

    /// Applies a 2x2 matrix: U * |self>
    pub fn apply_matrix(&self, matrix: &Matrix2x2) -> Self {
        let a = matrix.data[0][0] * self.alpha + matrix.data[0][1] * self.beta;
        let b = matrix.data[1][0] * self.alpha + matrix.data[1][1] * self.beta;
        Self::new(a, b)
    }
}

/// Error types for weak measurement operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeakMeasurementError {
    OrthogonalPostSelection,
    ZeroStateNorm,
}

/// Calculates the weak value of an operator A for a given pre-selected state |psi_i> and post-selected state |psi_f>.
/// A_w = <psi_f | A | psi_i> / <psi_f | psi_i>
pub fn calculate_weak_value(
    initial: &QubitState,
    final_state: &QubitState,
    operator: &Matrix2x2,
) -> Result<Complex64, WeakMeasurementError> {
    let norm_i = initial.alpha.norm_sqr() + initial.beta.norm_sqr();
    let norm_f = final_state.alpha.norm_sqr() + final_state.beta.norm_sqr();
    if norm_i < 1e-15 || norm_f < 1e-15 {
        return Err(WeakMeasurementError::ZeroStateNorm);
    }

    let overlap = final_state.inner_product(initial);
    if libm::fabs(overlap.re) < 1e-15 && libm::fabs(overlap.im) < 1e-15 {
        return Err(WeakMeasurementError::OrthogonalPostSelection);
    }

    let state_after_op = initial.apply_matrix(operator);
    let numerator = final_state.inner_product(&state_after_op);

    Ok(numerator / overlap)
}

/// Models the atomic excitation/dwell duration tau_dwell of a photon in a resonant atomic medium.
/// In standard interactions, tau_dwell > 0. Under specific pre- and post-selection, weak value measurement
/// yields negative dwell times (tau_dwell < 0).
/// This function calculates tau_dwell based on a weak value approach where the dwell time is proportional
/// to the real part of the weak value of the projector onto the excited state.
/// We model the projector onto the excited state |e><e| as a 2x2 matrix projector.
/// tau_dwell = t_0 * Re( P_e^w ), where P_e is the projector of the excited state.
pub fn calculate_weak_dwell_time(
    initial: &QubitState,
    final_state: &QubitState,
    excited_projector: &Matrix2x2,
    t_0: f64,
) -> Result<f64, WeakMeasurementError> {
    let wv = calculate_weak_value(initial, final_state, excited_projector)?;
    Ok(t_0 * wv.re)
}

/// Validates whether a given negative dwell time constitutes a valid physical weak-measurement observable.
/// Relies on post-selection overlap and non-orthogonality.
pub fn validate_negative_dwell_physicality(
    initial: &QubitState,
    final_state: &QubitState,
    dwell_time: f64,
) -> bool {
    let overlap_sq = final_state.inner_product(initial).norm_sqr();
    let overlap = libm::sqrt(overlap_sq);
    // To be a physically measurable weak value, the pre- and post-selected states must have non-zero overlap
    // but can be near-orthogonal (which yields large/negative amplification).
    // Extreme orthogonality (overlap < 1e-10) makes the weak value extremely noisy / unphysical due to state collapse.
    overlap > 1e-8 && dwell_time < 0.0
}
