/// Custom error representing safety boundary violations on the macroscopic level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacroScaleBoundaryViolation {
    EntropyDecreaseDetected,
    SuperluminalSignalDetected,
    MacroscopicParticleCountViolation,
}

/// Hard safety bounds for macroscopic quantum-state-reversal simulations.
/// Enforces delta_S_macro >= 0, signal velocity <= c, and number of particles <= 10.
pub struct MacroGuard {
    pub speed_of_light: f64, // Constant c
    pub max_quantum_particles: usize,
}

impl MacroGuard {
    pub const DEFAULT: Self = Self {
        speed_of_light: 299_792_458.0, // m/s
        max_quantum_particles: 10,     // strict bounds on particle count
    };

    /// Checks if the state transition violates macroscopic safety bounds.
    /// Returns Ok(()) if the transition is physically allowed, or a MacroScaleBoundaryViolation error.
    pub fn enforce_bounds(
        &self,
        delta_entropy: f64,
        signal_velocity: f64,
        particle_count: usize,
    ) -> Result<(), MacroScaleBoundaryViolation> {
        // Enforce the Second Law of Thermodynamics macro-scale guard
        if particle_count > self.max_quantum_particles && delta_entropy < 0.0 {
            return Err(MacroScaleBoundaryViolation::EntropyDecreaseDetected);
        }

        // Relativistic guard: no faster-than-light signaling
        if signal_velocity > self.speed_of_light {
            return Err(MacroScaleBoundaryViolation::SuperluminalSignalDetected);
        }

        // Hard macro-scale limit: we refuse to execute quantum rewinding simulations
        // on macroscopic states (defined here as exceeding the particle threshold).
        if particle_count > self.max_quantum_particles {
            return Err(MacroScaleBoundaryViolation::MacroscopicParticleCountViolation);
        }

        Ok(())
    }
}
