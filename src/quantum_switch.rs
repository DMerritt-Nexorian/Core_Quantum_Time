#![allow(clippy::needless_range_loop)]

pub use crate::weak_measurement::{Matrix2x2, QubitState};
use num_complex::Complex64;

/// Helper to create a complex number from polar coordinates in `#![no_std]`.
#[inline]
pub fn from_polar(r: f64, theta: f64) -> Complex64 {
    Complex64::new(r * libm::cos(theta), r * libm::sin(theta))
}

/// Represents a Hamiltonian for a 2-level quantum system (qubit).
/// H = d0 * I + dx * sigma_x + dy * sigma_y + dz * sigma_z
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hamiltonian {
    pub d0: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Hamiltonian {
    pub fn new(d0: f64, dx: f64, dy: f64, dz: f64) -> Self {
        Self { d0, dx, dy, dz }
    }

    /// Converts the Hamiltonian parameters to a concrete Matrix2x2.
    pub fn to_matrix(&self) -> Matrix2x2 {
        let r00 = Complex64::new(self.d0 + self.dz, 0.0);
        let r01 = Complex64::new(self.dx, -self.dy);
        let r10 = Complex64::new(self.dx, self.dy);
        let r11 = Complex64::new(self.d0 - self.dz, 0.0);
        Matrix2x2::new([[r00, r01], [r10, r11]])
    }

    /// Computes the exact time evolution operator U(t) = exp(-i * H * t).
    pub fn evolve_operator(&self, t: f64) -> Matrix2x2 {
        let d_norm = libm::sqrt(self.dx * self.dx + self.dy * self.dy + self.dz * self.dz);
        let exp_phase = from_polar(1.0, -self.d0 * t);

        if d_norm < 1e-15 {
            // exp(-i * d0 * t) * I
            Matrix2x2::IDENTITY.scale(exp_phase)
        } else {
            let cos_val = libm::cos(d_norm * t);
            let sin_val = libm::sin(d_norm * t);

            // M = cos(d*t)*I - i*sin(d*t)*(d_vec . sigma)/d
            let coeff = sin_val / d_norm;
            // -i * coeff
            let m_coeff = Complex64::new(0.0, -coeff);

            let m00 = Complex64::new(cos_val, 0.0) + m_coeff * self.dz;
            let m01 = m_coeff * Complex64::new(self.dx, -self.dy);
            let m10 = m_coeff * Complex64::new(self.dx, self.dy);
            let m11 = Complex64::new(cos_val, 0.0) - m_coeff * self.dz;

            let m = Matrix2x2::new([[m00, m01], [m10, m11]]);
            m.scale(exp_phase)
        }
    }

    /// Computes the exact time restoration/rewind operator R(H, t) = exp(i * H * t).
    pub fn restoration_operator(&self, t: f64) -> Matrix2x2 {
        let d_norm = libm::sqrt(self.dx * self.dx + self.dy * self.dy + self.dz * self.dz);
        let exp_phase = from_polar(1.0, self.d0 * t);

        if d_norm < 1e-15 {
            Matrix2x2::IDENTITY.scale(exp_phase)
        } else {
            let cos_val = libm::cos(d_norm * t);
            let sin_val = libm::sin(d_norm * t);

            let coeff = sin_val / d_norm;
            let m_coeff = Complex64::new(0.0, coeff);

            let m00 = Complex64::new(cos_val, 0.0) + m_coeff * self.dz;
            let m01 = m_coeff * Complex64::new(self.dx, -self.dy);
            let m10 = m_coeff * Complex64::new(self.dx, self.dy);
            let m11 = Complex64::new(cos_val, 0.0) - m_coeff * self.dz;

            let m = Matrix2x2::new([[m00, m01], [m10, m11]]);
            m.scale(exp_phase)
        }
    }
}

/// Represents the joint state of a control qubit and a target qubit.
/// State is expressed in basis {|00>, |01>, |10>, |11>}.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointState {
    pub data: [Complex64; 4],
}

impl JointState {
    pub fn new(data: [Complex64; 4]) -> Self {
        Self { data }
    }

    /// Creates a separable joint state |control> \otimes |target>.
    pub fn separable(control: &QubitState, target: &QubitState) -> Self {
        Self {
            data: [
                control.alpha * target.alpha,
                control.alpha * target.beta,
                control.beta * target.alpha,
                control.beta * target.beta,
            ],
        }
    }

    pub fn normalize(&self) -> Self {
        let mut norm_sq = 0.0;
        for val in &self.data {
            norm_sq += val.norm_sqr();
        }
        if norm_sq > 1e-15 {
            let norm = libm::sqrt(norm_sq);
            let mut normalized = [Complex64::new(0.0, 0.0); 4];
            for i in 0..4 {
                normalized[i] = self.data[i] / norm;
            }
            Self::new(normalized)
        } else {
            *self
        }
    }

    pub fn inner_product(&self, other: &Self) -> Complex64 {
        let mut sum = Complex64::new(0.0, 0.0);
        for i in 0..4 {
            sum += self.data[i].conj() * other.data[i];
        }
        sum
    }
}

/// Represents a 4x4 complex matrix acting on the joint control-target space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub data: [[Complex64; 4]; 4],
}

impl Matrix4x4 {
    pub const ZERO: Self = Self {
        data: [[Complex64::new(0.0, 0.0); 4]; 4],
    };

    pub const IDENTITY: Self = Self {
        data: [
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
            ],
        ],
    };

    pub fn new(data: [[Complex64; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = Complex64::new(0.0, 0.0);
                for k in 0..4 {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res[i][j] = sum;
            }
        }
        Self::new(res)
    }

    pub fn conjugate_transpose(&self) -> Self {
        let mut res = [[Complex64::new(0.0, 0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = self.data[j][i].conj();
            }
        }
        Self::new(res)
    }

    pub fn apply(&self, state: &JointState) -> JointState {
        let mut res = [Complex64::new(0.0, 0.0); 4];
        for i in 0..4 {
            let mut sum = Complex64::new(0.0, 0.0);
            for j in 0..4 {
                sum += self.data[i][j] * state.data[j];
            }
            res[i] = sum;
        }
        JointState::new(res)
    }
}

/// Builds the Quantum Switch operator over indefinite causal orders:
/// U_switch = |0><0| \otimes (U_B U_A) + |1><1| \otimes (U_A U_B)
pub fn build_quantum_switch(u_a: &Matrix2x2, u_b: &Matrix2x2) -> Matrix4x4 {
    let u_b_u_a = u_b.mul(u_a);
    let u_a_u_b = u_a.mul(u_b);

    let mut data = [[Complex64::new(0.0, 0.0); 4]; 4];

    // Top-left 2x2 block corresponds to control state |0>
    data[0][0] = u_b_u_a.data[0][0];
    data[0][1] = u_b_u_a.data[0][1];
    data[1][0] = u_b_u_a.data[1][0];
    data[1][1] = u_b_u_a.data[1][1];

    // Bottom-right 2x2 block corresponds to control state |1>
    data[2][2] = u_a_u_b.data[0][0];
    data[2][3] = u_a_u_b.data[0][1];
    data[3][2] = u_a_u_b.data[1][0];
    data[3][3] = u_a_u_b.data[1][1];

    Matrix4x4::new(data)
}
