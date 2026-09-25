# DRAWINGS AND DIAGRAMS: AHARONOV SIMULATION ENGINE

## 1. Weak Measurement Optical Path & Post-Selection Setup
The following schematic illustrates the physical weak interaction flow of a photon passing through a resonant Rubidium vapor cell under post-selection filtering.

```
       [Photon Source]
              │
              ▼
    ┌──────────────────┐
    │  Pre-Selection   │  --> Prepares state |psi_i> = cos(theta_i)|g> + sin(theta_i)|e>
    └──────────────────┘
              │
              ▼
    ┌──────────────────┐
    │  Atomic Cloud    │  --> Resonant Rubidium vapor cell
    │ (Rb-87 Vapor)    │  --> Weak interaction (g -> 0)
    └──────────────────┘
              │
              ▼
    ┌──────────────────┐
    │  Post-Selection  │  --> Near-orthogonal filter |psi_f> (cos(theta_f)|g> + sin(theta_f)|e>)
    └──────────────────┘
              │
              ▼
     [Optical Detector]  --> Measures negative dwell time observable (tau_dwell < 0)
```

---

## 2. Quantum Switch Indefinite Causal Circuit
The quantum switch superimposes two operational sequences $U_A$ and $U_B$ using a control qubit to evaluate indefinite causal orders.

```
                    ┌─────────┐
Control Qubit: ───●──┤ Hadamard├────────────────────────────●── [Measure Control]
                  │  └─────────┘                            │
                  │         ┌─────┐                 ┌─────┐ │
                  ├──[0]───┤ U_A ├──────[1]───────┤ U_B ├─┤
                  │         └─────┘                 └─────┘ │
Target Qubit:  ───┼─────────────────────────────────────────┼── [Output State |psi>]
                  │         ┌─────┐                 ┌─────┐ │
                  └──[1]───┤ U_B ├──────[0]───────┤ U_A ├─┘
                            └─────┘                 └─────┘
```

---

## 3. Unitary State Restoration (Rewinding) Transition Flow
The state restoration protocol applies $R(\hat{H}, t) = \exp(i \hat{H} t)$ to rewind an evolved state back to its initial state without measurement collapse.

```
         ┌───────────────────────────────┐
         │      Initial State |psi_0>    │
         └──────────────┬────────────────┘
                        │
                        │ Evolve under H for time t: U(H, t) = exp(-i*H*t)
                        ▼
         ┌───────────────────────────────┐
         │      Evolved State |psi(t)>   │
         └──────────────┬────────────────┘
                        │
                        │ Apply Restoration R(H, t) = exp(i*H*t) via Quantum Switch
                        ▼
         ┌───────────────────────────────┐
         │  Restored State |psi_rewound> │  <-- (Unitary Fidelity F = 1.0)
         └───────────────────────────────┘
```
