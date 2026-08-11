# PHYSICS SPECIFICATION: QUANTUM NEGATIVE DWELL TIME & STATE RESTORATION PROTOCOLS

## 1. Introduction and Foundations
The `CORE_QUANTUM_TIME` computational engine models two primary quantum phenomena:
- **Negative Dwell Time via Weak Measurement**: The physical modeling of atomic excitation duration during photon transition, where pre- and post-selection results in a negative value ($\tau_{\text{dwell}} < 0$).
- **Quantum Switch-based State Restoration ("Quantum Rewind")**: Reversing target qubit states back to their initial configuration without intermediate state measurements, by placing operations on superposed paths.

---

## 2. Weak Value Theory and Negative Dwell Time
In standard quantum mechanics, eigenvalues of Hermitian operators define outcomes of strong measurements. When a measurement interaction is weak, we can obtain "weak values" outside the standard eigenvalue spectrum.

### Mathematical Formulation
Let $|\psi_i\rangle$ be the initial pre-selected state of the system, and $|\psi_f\rangle$ be the post-selected state. For any physical operator $\hat{A}$, the weak value $A_w$ is given by:

$$A_w = \frac{\langle \psi_f | \hat{A} | \psi_i \rangle}{\langle \psi_f | \psi_i \rangle}$$

If the projection onto the excited state is given by the projector $\hat{P}_e = |e\rangle\langle e|$, we can define the weak dwell time $\tau_{\text{dwell}}$ inside the resonant medium as:

$$\tau_{\text{dwell}} = t_0 \cdot \text{Re}(P_e^w) = t_0 \cdot \text{Re}\left( \frac{\langle \psi_f | \hat{P}_e | \psi_i \rangle}{\langle \psi_f | \psi_i \rangle} \right)$$

where $t_0$ is the standard excitation duration constant.

### Derivation of Negative Dwell Time
Suppose we prepare the pre-selected state:
$$|\psi_i\rangle = \cos(\theta_i)|g\rangle + \sin(\theta_i)|e\rangle$$
And post-select near-orthogonal:
$$|\psi_f\rangle = \cos(\theta_f)|g\rangle + \sin(\theta_f)|e\rangle$$

The overlap (denominator) is:
$$\langle \psi_f | \psi_i \rangle = \cos(\theta_f)\cos(\theta_i) + \sin(\theta_f)\sin(\theta_i)$$

The numerator with excited state projector $\hat{P}_e = |e\rangle\langle e|$ is:
$$\langle \psi_f | \hat{P}_e | \psi_i \rangle = \sin(\theta_f)\sin(\theta_i)$$

If we select $\theta_i = 0.1$ and $\theta_f = -1.45$:
- $\sin(\theta_i) \approx 0.10, \sin(\theta_f) \approx -0.99 \implies \text{numerator} \approx -0.099$
- $\langle \psi_f | \psi_i \rangle \approx 0.12 \cdot 0.995 + (-0.99) \cdot 0.10 = 0.119 - 0.099 = 0.020$

Thus, the real part of the weak value is:
$$\text{Re}(P_e^w) \approx \frac{-0.099}{0.020} = -4.95$$

Yielding a negative dwell time:
$$\tau_{\text{dwell}} = t_0 \cdot (-4.95) < 0$$

This is a physical observable, representing a negative excitation delay where the atom acts as if it was "un-excited" or spend negative time in the excited state under interference conditions.

---

## 3. Quantum Switch & Time Reversal Protocol
The quantum switch allows two operations, $U_A$ and $U_B$, to be applied in an indefinite causal order, superposed by a control qubit:

$$U_{\text{switch}} = |0\rangle\langle 0| \otimes (U_B U_A) + |1\rangle\langle 1| \otimes (U_A U_B)$$

### State Restoration Operator $R(\hat{H}, t)$
To rewind a system's evolution under Hamiltonian $\hat{H}$ for time $t$, we seek an operator $R(\hat{H}, t)$ such that:

$$R(\hat{H}, t) U(\hat{H}, t) = I$$

Since $U(\hat{H}, t) = \exp(-i \hat{H} t)$, the exact state restoration operator is:

$$R(\hat{H}, t) = \exp(i \hat{H} t) = U^\dagger(\hat{H}, t)$$

Applying this operator maps any evolved state $|\psi(t)\rangle$ back to the original $|\psi_0\rangle$ with unitary fidelity $F = 1.0$:

$$|\psi_{\text{rewound}}\rangle = R(\hat{H}, t)|\psi(t)\rangle = \exp(i \hat{H} t)\exp(-i \hat{H} t)|\psi_0\rangle = |\psi_0\rangle$$
