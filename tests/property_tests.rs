use core_quantum_time_engine::quantum_switch::Hamiltonian;
use core_quantum_time_engine::weak_measurement::Matrix2x2;
use num_complex::Complex64;
use proptest::prelude::*;

fn check_unitary(m: &Matrix2x2) -> bool {
    let m_dag = m.conjugate_transpose();
    let identity_candidate = m.mul(&m_dag);

    // Check that identity_candidate is extremely close to Matrix2x2::IDENTITY
    let diff_00 = (identity_candidate.data[0][0] - Complex64::new(1.0, 0.0)).norm_sqr();
    let diff_01 = (identity_candidate.data[0][1] - Complex64::new(0.0, 0.0)).norm_sqr();
    let diff_10 = (identity_candidate.data[1][0] - Complex64::new(0.0, 0.0)).norm_sqr();
    let diff_11 = (identity_candidate.data[1][1] - Complex64::new(1.0, 0.0)).norm_sqr();

    diff_00 < 1e-12 && diff_01 < 1e-12 && diff_10 < 1e-12 && diff_11 < 1e-12
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn test_unitary_preservation(
        d0 in -10.0..10.0f64,
        dx in -10.0..10.0f64,
        dy in -10.0..10.0f64,
        dz in -10.0..10.0f64,
        t in 0.0..100.0f64,
    ) {
        let h = Hamiltonian::new(d0, dx, dy, dz);
        let u = h.evolve_operator(t);
        prop_assert!(check_unitary(&u));

        let r = h.restoration_operator(t);
        prop_assert!(check_unitary(&r));
    }
}
