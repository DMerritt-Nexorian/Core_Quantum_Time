# AHARANOV: Quantum State Temporal Reversal & Weak Measurement Engine

**Aharonov** is a safety-critical, high-performance computational engine designed to model, verify, and simulate particle-scale temporal state manipulation, weak-measurement dynamics, and negative dwell states.

Engineered for TRL-9 software readiness, the core engine is written in 100% pure, `#![no_std]` compliant Rust (with optional `alloc` feature flags). This enables zero-allocation, deterministic execution on real-time quantum control microcontrollers, bare-metal signal processing units, and high-throughput quantum error correction (QEC) hardware interfaces.

---

## 📜 End User License Agreement (EULA) & Terms of Use

### **STRICT DEVELOPER EVALUATION LICENSE**
Copyright © 2025 **Aharonov Software** (or its licensors). All Rights Reserved.

**IMPORTANT NOTICE**: This public repository (`aharonov`) contains interface definitions, public trait abstractions, and reference simulation logic intended **SOLELY FOR NON-COMMERCIAL, ACADEMIC, AND RESEARCH EVALUATION PURPOSES**.

1. **Non-Commercial Academic Scope**: You are granted a limited, non-exclusive, non-transferable, revocable license to view, clone, compile, and run this codebase for academic research, peer review, and non-commercial evaluation.
2. **Commercial & Production Prohibition**: Any commercial, enterprise, production, or revenue-generating use of this software, its derivative works, or its underlying computational algorithms is **STRICTLY PROHIBITED** without a valid, written Commercial Production License issued by Aharonov Software.
3. **Proprietary Acceleration Engine**: The high-performance SIMD acceleration loops, multi-threaded task schedulers, hardware FPGA synthesis pipelines, and advanced tensor contraction solvers remain closed-source in a private enterprise repository.
4. **Licensing Inquiries**: To acquire a Commercial Production License, enterprise support, or private source access, contact:
   - **Entity**: Aharonov Software
   - **Email**: `licensing@aharonov.io`

---

## 🔬 Key Quantum Mechanics Modules

### 1. Quantum "Negative Time" Dwell States (`src/weak_measurement.rs`)
The engine models photon propagation through resonant atomic mediums using weak measurement mechanics. By evaluating the weak value of the atomic excitation projection operator:

$$\hat{P}_e = |e\rangle\langle e|$$

The engine calculates the effective atomic excitation duration $\tau_{\text{dwell}}$. Under post-selected near-orthogonal quantum states ($|\langle \psi_f | \psi_i \rangle| \ll 1$), the calculated dwell time yields a physical negative observable:

$$\tau_{\text{dwell}} = t_0 \cdot \text{Re}\left( \frac{\langle \psi_f | \hat{P}_e | \psi_i \rangle}{\langle \psi_f | \psi_i \rangle} \right) < 0$$

This models the verified physical phenomenon where photons exit an atomic cloud medium prior to the completion of entering pulse excitation, without violating causality or phase velocity limits.

### 2. Quantum Time Reversal Protocols ("Quantum Rewind") (`src/quantum_switch.rs`)
To achieve non-destructive qubit state restoration without intermediate state collapse, Aharonov implements a quantum switch protocol operating over indefinite causal orders.

By superimposing dual evolutionary pathways (Hamiltonians $H_A$ and $H_B$) using a control qubit, the engine executes the unitary reversal operator:

$$R(\hat{H}, t) = \exp(i \hat{H} t) = U^\dagger(\hat{H}, t)$$

This enables rewinding or fast-forwarding an unknown qubit state $|\psi(t)\rangle$ back to its target initial state $|\psi(0)\rangle$ with near-perfect fidelity ($F \approx 1.0$) for real-time quantum error correction.

### 3. Public Trait Abstractions (`src/traits.rs`)
The public `aharonov` crate exposes strict, zero-cost trait abstractions:
- `StateVector`: Defines inner products, normalization, and norm calculations.
- `WeakMeasurer`: Interface for evaluating weak values and negative dwell times.
- `QuantumSwitch`: Interface for indefinite causal order superpositions and unitary state rewinding.

Public reference implementations (`ReferenceWeakMeasurer`, `ReferenceQuantumSwitch`) allow researchers to integrate and test custom hardware or simulation adapters seamlessly.

### 4. Macro-Scale Safety Guard & Bottleneck Enforcement (`src/macro_guard.rs`)
To prevent unphysical extrapolation or invalid macro-scale state leaks, Aharonov hard-codes fundamental thermodynamic and relativistic boundary limits (`MacroGuard`).

The engine instantly halts execution and triggers a `MacroScaleBoundaryViolation` error if any simulation attempts to violate:
- **Second Law of Thermodynamics**: $\Delta S_{\text{macro}} \ge 0$ (State reversal is strictly bounded to isolated subatomic particles, $N \le 10$).
- **Relativistic Signal Bound**: $v_{\text{signal}} \le c$ (Prevents classical information transmission or matter transport back through time).

---

## 📁 Repository Structure

```
aharonov/
├── Cargo.toml                  # Crate manifest (#![no_std] dependencies)
├── .github/
│   └── workflows/
│       └── ci.yml              # Automated CI/CD pipeline (fmt, clippy, test)
├── src/                        # Core library source (100% #![no_std])
│   ├── lib.rs                  # Primary module declarations, EULA notice & public API
│   ├── traits.rs               # StateVector, WeakMeasurer, and QuantumSwitch traits
│   ├── weak_measurement.rs     # Weak values & negative dwell calculations
│   ├── quantum_switch.rs       # Indefinite causal order state rewind engine
│   └── macro_guard.rs          # Thermodynamic & relativistic safety gates
├── tests/                      # Verification and test suites
│   ├── trait_tests.rs          # Interface and trait abstraction unit tests
│   ├── weak_measurement_tests.rs # Dwell time precision unit tests
│   ├── quantum_switch_tests.rs # State restoration fidelity unit tests
│   ├── adversarial_tests.rs    # Macro-scale violation safety boundary tests
│   └── property_tests.rs       # Proptest property fuzzing for unitary preservation
└── docs/                       # Masterclass technical & architectural specifications
    ├── PHYSICS_SPECIFICATION.md   # Rigorous mathematical derivations & quantum proofs
    ├── MACRO_LIMITATION_BOUNDS.md # Thermodynamic & relativistic constraints
    └── DRAWINGS_AND_DIAGRAMS.md   # ASCII state machines & circuit flows
```

---

## 🛠️ Verification, Testing & Build Protocols

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
Enforce strict quality control with zero compiler warnings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```
