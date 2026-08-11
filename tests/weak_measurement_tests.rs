use core_quantum_time_engine::weak_measurement::{
    Matrix2x2, QubitState, calculate_weak_dwell_time, validate_negative_dwell_physicality,
};
use num_complex::Complex64;

#[test]
fn test_negative_dwell_time_generation() {
    // Construct standard pre- and post-selected states.
    // If post-selection |psi_f> is close to orthogonal to |psi_i>, we get anomalous weak values.
    // Let's set up:
    // |psi_i> = cos(theta_i)|0> + sin(theta_i)|1>
    // |psi_f> = cos(theta_f)|0> + sin(theta_f)|1>
    // P_e = |1><1|
    // then <psi_f| P_e |psi_i> = sin(theta_f)*sin(theta_i)
    // and <psi_f|psi_i> = cos(theta_f)*cos(theta_i) + sin(theta_f)*sin(theta_i)
    // If we choose theta_i = 0.1, theta_f = -1.45 (near-orthogonal)
    // sin(theta_f) approx -0.99, sin(theta_i) approx 0.10 => numerator = -0.099
    // denominator = cos(-1.45)*cos(0.1) + sin(-1.45)*sin(0.1) = 0.12 * 0.995 + (-0.99) * 0.10 = 0.119 - 0.099 = 0.020
    // Weak value = numerator / denominator = -0.099 / 0.020 = -4.95 (negative!)
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

    // Operator: projector on state |e>
    let p_e = Matrix2x2::new([
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    ]);

    let t_0 = 10.0; // Standard dwell excitation scale in nanoseconds
    let dwell_res = calculate_weak_dwell_time(&initial_state, &final_state, &p_e, t_0);
    assert!(dwell_res.is_ok());
    let dwell_val = dwell_res.unwrap();
    assert!(
        dwell_val < 0.0,
        "Dwell time must be negative under post-selection: {}",
        dwell_val
    );

    // Verify physicality
    let physical = validate_negative_dwell_physicality(&initial_state, &final_state, dwell_val);
    assert!(physical, "Negative dwell time must be physical");
}

#[test]
fn test_orthogonal_rejection() {
    let initial = QubitState::new(Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0));
    let final_state = QubitState::new(Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0));
    let p_e = Matrix2x2::IDENTITY;

    let res = calculate_weak_dwell_time(&initial, &final_state, &p_e, 1.0);
    assert_eq!(res, Err(core_quantum_time_engine::weak_measurement::WeakMeasurementError::OrthogonalPostSelection));
}
