use aharonov::weak_measurement::{
    BasicWeakMeasurer, Matrix2x2, QubitState, WeakMeasurementError, WeakMeasurer,
    validate_negative_dwell_physicality,
};
use num_complex::Complex64;

#[test]
fn test_negative_dwell_time_generation() {
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
    let measurer = BasicWeakMeasurer::new();
    let dwell_res = measurer.post_select_state(&initial_state, &final_state, &p_e, t_0);
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

    let measurer = BasicWeakMeasurer::new();
    let res = measurer.post_select_state(&initial, &final_state, &p_e, 1.0);
    assert_eq!(res, Err(WeakMeasurementError::OrthogonalPostSelection));
}
