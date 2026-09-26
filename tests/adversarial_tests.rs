use aharonov::macro_guard::{MacroGuard, MacroScaleBoundaryViolation};

#[test]
fn test_macro_guard_legitimate_transition() {
    let guard = MacroGuard::DEFAULT;

    // Normal micro-scale transition: 1 particle, positive entropy change, subluminal velocity
    let res = guard.enforce_bounds(0.001, 1500.0, 1);
    assert!(res.is_ok());
}

#[test]
fn test_macro_guard_entropy_violation() {
    let guard = MacroGuard::DEFAULT;

    // Try to decrease entropy on a macroscopic system (e.g. 50 particles)
    let res = guard.enforce_bounds(-2.5, 1500.0, 50);
    assert_eq!(
        res,
        Err(MacroScaleBoundaryViolation::EntropyDecreaseDetected)
    );
}

#[test]
fn test_macro_guard_superluminal_violation() {
    let guard = MacroGuard::DEFAULT;

    // Try to send information faster than the speed of light
    let res = guard.enforce_bounds(0.0, 300_000_000.0, 1);
    assert_eq!(
        res,
        Err(MacroScaleBoundaryViolation::SuperluminalSignalDetected)
    );
}

#[test]
fn test_macro_guard_macroscopic_count_violation() {
    let guard = MacroGuard::DEFAULT;

    // Attempting state reversal simulation on 1,000,000 particles (macro-scale matter)
    let res = guard.enforce_bounds(0.0, 1.0, 1_000_000);
    assert_eq!(
        res,
        Err(MacroScaleBoundaryViolation::MacroscopicParticleCountViolation)
    );
}
