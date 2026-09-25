use aharonov::quantum_switch::{BasicQuantumSwitch, Hamiltonian, JointState, QuantumSwitch};
use aharonov::weak_measurement::QubitState;
use num_complex::Complex64;

#[test]
fn test_exact_state_restoration_fidelity() {
    // Define an initial qubit state |\psi_0>
    let psi_0 = QubitState::new(
        Complex64::new(0.6, 0.0),
        Complex64::new(0.0, 0.8), // Normalized since 0.6^2 + 0.8^2 = 1.0
    );

    // Set up a random realistic Hamiltonian H
    let h = Hamiltonian::new(1.2, -0.5, 0.8, 2.3);

    let t = 2.45; // evolution time

    // Forward evolution: U(t) * |\psi_0>
    let u_forward = h.evolve_operator(t);
    let psi_t = psi_0.apply_matrix(&u_forward);

    // State restoration (Rewind): R(H, t) * |\psi_t>
    let switch_engine = BasicQuantumSwitch::new();
    let psi_rewound = switch_engine.rewind_causal_order(&h, &psi_t, t);

    // Calculate fidelity F = |<psi_0 | psi_rewound>|^2
    let overlap = psi_0.inner_product(&psi_rewound);
    let fidelity = overlap.norm_sqr();

    let diff = libm::fabs(fidelity - 1.0);
    assert!(
        diff < 1e-9,
        "Fidelity difference is larger than 1e-9: {} (fidelity: {})",
        diff,
        fidelity
    );
}

#[test]
fn test_quantum_switch_indefinite_causal_order() {
    // Control in superposition (|0> + |1>) / sqrt(2)
    let control = QubitState::new(
        Complex64::new(1.0 / libm::sqrt(2.0), 0.0),
        Complex64::new(1.0 / libm::sqrt(2.0), 0.0),
    );

    // Target state |0>
    let target = QubitState::new(Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0));

    // Two distinct non-commuting unitary operations (rotation matrices or Pauli operators)
    // U_A: Rx(pi/4)
    let h_a = Hamiltonian::new(0.0, libm::acos(-1.0) / 4.0, 0.0, 0.0);
    let u_a = h_a.evolve_operator(1.0);

    // U_B: Ry(pi/4)
    let h_b = Hamiltonian::new(0.0, 0.0, libm::acos(-1.0) / 4.0, 0.0);
    let u_b = h_b.evolve_operator(1.0);

    // Build switch using trait method
    let switch_engine = BasicQuantumSwitch::new();
    let switch = switch_engine.superimpose_operations(&u_a, &u_b);

    let joint_init = JointState::separable(&control, &target);
    let joint_final = switch.apply(&joint_init);

    // Verify the state is normalized
    let norm = joint_final.inner_product(&joint_final);
    assert!(libm::fabs(norm.re - 1.0) < 1e-12);
}
