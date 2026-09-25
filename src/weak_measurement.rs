#![allow(clippy::needless_range_loop)]

use num_complex::Complex64;

/// Stack-bounded 2x2 complex matrix for qubit operations.
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

/// 2D complex state vector (qubit).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QubitState {
    pub alpha: Complex64,
    pub beta: Complex64,
}

impl QubitState {
    pub fn new(alpha: Complex64, beta: Complex64) -> Self {
        Self { alpha, beta }
    }

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

    pub fn inner_product(&self, other: &Self) -> Complex64 {
        self.alpha.conj() * other.alpha + self.beta.conj() * other.beta
    }

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

/// Strict public trait interface for weak measurement interaction and state post-selection.
pub trait WeakMeasurer {
    /// Evaluates the weak value A_w = <psi_f | A | psi_i> / <psi_f | psi_i>
    fn register_weak_interaction(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        operator: &Matrix2x2,
    ) -> Result<Complex64, WeakMeasurementError>;

    /// Computes post-selected dwell time tau_dwell = t_0 * Re(P_e^w)
    fn post_select_state(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        excited_projector: &Matrix2x2,
        t_0: f64,
    ) -> Result<f64, WeakMeasurementError>;
}

/// Basic unoptimized pure-Rust reference implementation of `WeakMeasurer`.
/// High-performance SIMD-accelerated statistical implementations remain private.
#[derive(Debug, Clone, Copy, Default)]
pub struct BasicWeakMeasurer;

impl BasicWeakMeasurer {
    pub fn new() -> Self {
        Self
    }
}

impl WeakMeasurer for BasicWeakMeasurer {
    fn register_weak_interaction(
        &self,
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

    fn post_select_state(
        &self,
        initial: &QubitState,
        final_state: &QubitState,
        excited_projector: &Matrix2x2,
        t_0: f64,
    ) -> Result<f64, WeakMeasurementError> {
        let wv = self.register_weak_interaction(initial, final_state, excited_projector)?;
        Ok(t_0 * wv.re)
    }
}

/// Legacy standalone function re-exports for backwards compatibility with tests.
pub fn calculate_weak_value(
    initial: &QubitState,
    final_state: &QubitState,
    operator: &Matrix2x2,
) -> Result<Complex64, WeakMeasurementError> {
    BasicWeakMeasurer.register_weak_interaction(initial, final_state, operator)
}

pub fn calculate_weak_dwell_time(
    initial: &QubitState,
    final_state: &QubitState,
    excited_projector: &Matrix2x2,
    t_0: f64,
) -> Result<f64, WeakMeasurementError> {
    BasicWeakMeasurer.post_select_state(initial, final_state, excited_projector, t_0)
}

pub fn validate_negative_dwell_physicality(
    initial: &QubitState,
    final_state: &QubitState,
    dwell_time: f64,
) -> bool {
    let overlap_sq = final_state.inner_product(initial).norm_sqr();
    let overlap = libm::sqrt(overlap_sq);
    overlap > 1e-8 && dwell_time < 0.0
}
