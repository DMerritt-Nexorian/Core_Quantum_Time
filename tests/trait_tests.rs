use aharonov::quantum_switch::{Hamiltonian, JointState};
use aharonov::traits::{
    QuantumSwitch, ReferenceQuantumSwitch, ReferenceWeakMeasurer, StateVector, WeakMeasurer,
};
use aharonov::weak_measurement::{Matrix2x2, QubitState};
use num_complex::Complex64;

#[test]
fn test_state_vector_trait_behavior() {
    let q = QubitState::new(Complex64::new(3.0, 0.0), Complex64::new(4.0, 0.0));
    assert_eq!(q.norm_sqr(), 25.0);

    let norm_q = q.normalize();
    assert_eq!(norm_q.norm_sqr(), 1.0);
    assert_eq!(norm_q.alpha, Complex64::new(0.6, 0.0));
    assert_eq!(norm_q.beta, Complex64::new(0.8, 0.0));
}

#[test]
fn test_weak_measurer_trait_implementation() {
    let measurer = ReferenceWeakMeasurer;
    let theta_i = 0.1;
    let theta_f = -1.45;
    let initial_state = QubitState::new(
        Complex64::new(libm::cos(theta_i), 0.0),
        Complex64::new(libm::sin(theta_i), 0.0),
    );
    let final_state = QubitState::new(
        Complex64::new(libm::cos(theta_f), 0.0),
        Complex64::new(libm::sin(theta_f), 0.0),
    );
    let p_e = Matrix2x2::new([
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    ]);

    let dwell = measurer
        .calculate_weak_dwell_time(&initial_state, &final_state, &p_e, 10.0)
        .unwrap();
    assert!(dwell < 0.0);
    assert!(measurer.validate_dwell_physicality(&initial_state, &final_state, dwell));
}

#[test]
fn test_quantum_switch_trait_implementation() {
    let switch_engine = ReferenceQuantumSwitch;
    let h = Hamiltonian::new(1.0, 0.5, -0.2, 0.8);
    let duration = 1.5;

    let u = switch_engine.evolve(&h, duration);
    let r = switch_engine.restore(&h, duration);

    // Verify U * R is Identity
    let identity_candidate = u.mul(&r);
    let diff = (identity_candidate.data[0][0] - Complex64::new(1.0, 0.0)).norm_sqr();
    assert!(diff < 1e-12);

    // Test restoration execution via trait
    let control = QubitState::new(
        Complex64::new(1.0 / libm::sqrt(2.0), 0.0),
        Complex64::new(1.0 / libm::sqrt(2.0), 0.0),
    );
    let target = QubitState::new(Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0));
    let joint = JointState::separable(&control, &target);

    let result = switch_engine.execute_restoration(&joint, &h, duration, 2);
    assert!(result.is_ok());
}
