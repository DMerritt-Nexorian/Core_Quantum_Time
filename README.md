# AHARONOV: High-Performance Quantum State Temporal Reversal & Weak Measurement Framework

> **Aharonov** is a safety-critical, high-precision computational framework designed for quantum state temporal manipulation, weak measurement post-selection, and indefinite causal order switch simulations.

Engineered for TRL-9 software readiness, **Aharonov** is written in 100% pure, `#![no_std]` compliant Rust. This public repository provides standard reference implementations and formal Rust trait abstractions (`WeakMeasurer`, `QuantumSwitch`, `StateVector`), enabling academic evaluation, research verification, and real-time quantum control microcontroller integration without allocating on the heap.

---

## 🔒 Developer License & EULA Notice

**IMPORTANT:** This public repository is governed by the **Aharonov Evaluation and Academic License (v1.0.0)**.

- **Non-Commercial & Academic Grant:** You are free to compile, study, and execute this codebase for personal education, non-commercial academic research, or internal evaluation.
- **Commercial & Production Restriction:** Production use, commercial deployments, cloud-hosted services (SaaS), or deployment in any revenue-generating activities is **strictly prohibited** without a paid commercial license.
- **Enterprise Upgrade:** High-performance multi-threaded scheduling, SIMD-vectorized state evolvers, cache-aligned state representations, and proprietary hardware drivers are available exclusively in the private production release. Contact Aharonov Software for commercial production licensing.

Please review the full license terms in [LICENSE.md](LICENSE.md).

---

## 🏛 Architecture & Public Trait Abstractions

The public crate exposes explicit Rust trait interfaces paired with basic, unoptimized reference implementations:

1. **Weak Measurement (`src/weak_measurement.rs`)**
   - **`WeakMeasurer` Trait**: Exposes `register_weak_interaction` and `post_select_state` interfaces for weak value amplification and negative dwell time calculations ($\tau_{\text{dwell}} < 0$).
   - **Reference Implementation**: `BasicWeakMeasurer` provides exact double-precision complex arithmetic.

2. **Quantum Switch & Time Reversal (`src/quantum_switch.rs`)**
   - **`QuantumSwitch` Trait**: Exposes `superimpose_operations` and `rewind_causal_order` interfaces for indefinite causal structure evaluation ($U_{\text{switch}} = |0\rangle\langle 0| \otimes U_B U_A + |1\rangle\langle 1| \otimes U_A U_B$).
   - **Reference Implementation**: `BasicQuantumSwitch` provides pure non-allocated unitary switch construction.

3. **State Vector Manipulations (`src/state_vector.rs`)**
   - **`StateVector` Trait**: Exposes `evolve_time` and `apply_shock_matrix` interfaces for unitary time evolution and perturbational shocks.
   - **Reference Implementation**: `BasicStateVector` models stack-bounded 2D/4D complex state vectors.

4. **Macroscopic Boundary Enforcement (`src/macro_guard.rs`)**
   - Hard-coded safety gates (`MacroGuard`) enforcing physical boundary constraints:
     - **Second Law of Thermodynamics**: $\Delta S_{\text{macro}} \ge 0$ for $N > 10$ particles.
     - **Special Relativity**: $v_{\text{signal}} \le c$.
     - **Particle Threshold**: Maximum 10 particles per microscopic quantum reversal block.

---

## 📚 Masterclass Documentation

For full mathematical proofs, quantum paradox resolutions, and performance benchmarks against Python (QuTiP/Qiskit) and C++ (QuEST), consult our technical specifications:

- **[docs/PHYSICS_SPECIFICATION.md](docs/PHYSICS_SPECIFICATION.md)**: Rigorous quantum derivations, weak measurement theory, resolution of measurement paradoxes, and microsecond latency comparison.
- **[docs/MACRO_LIMITATION_BOUNDS.md](docs/MACRO_LIMITATION_BOUNDS.md)**: Thermodynamic entropy bounds, special relativity constraints, and scientific citations (Aharonov et al. 1988, Chiribella et al. 2013).
- **[docs/DRAWINGS_AND_DIAGRAMS.md](docs/DRAWINGS_AND_DIAGRAMS.md)**: Optical flow diagrams, quantum switch schematics, and state-transition flows.

---

## 🛠 Compilation and Verification Protocols

### Bare-Metal ARM Compilation (`#![no_std]`)
Verify zero-allocation compatibility on bare-metal ARM Cortex-M hardware:
```bash
cargo build --target thumbv7em-none-eabi --no-default-features
```

### Full Test Suite Execution
Run all unit tests, integration tests, and property-based fuzzing suites:
```bash
cargo test --all-targets --all-features
```

### Static Analysis & Linter Audit
```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```
