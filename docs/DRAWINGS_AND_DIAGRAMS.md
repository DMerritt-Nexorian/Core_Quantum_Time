# DRAWINGS AND DIAGRAMS: CORE_QUANTUM_TIME SIMULATION ENGINE

## 1. Weak Measurement Optical Path for Negative Dwell Simulation
The following schematic describes the physical flow of a photon passing through an atomic medium with pre- and post-selection stages.

```
       [Photon Source]
              │
              ▼
    ┌──────────────────┐
    │  Pre-Selection   │  --> Prepares initial state |psi_i>
    └──────────────────┘
              │
              ▼
    ┌──────────────────┐
    │  Atomic Cloud    │  --> Resonant atomic medium
    │ (Rubidium Vapor) │  --> Weak interaction with probe laser
    └──────────────────┘
              │
              ▼
    ┌──────────────────┐
    │  Post-Selection  │  --> Near-orthogonal filter |psi_f>
    └──────────────────┘
              │
              ▼
     [Optical Detector]  --> Measures negative dwell time (tau_dwell < 0)
```

---

## 2. Quantum Switch Circuit
The quantum switch implements an indefinite causal order of operations $U_A$ and $U_B$ controlled by a routing/control qubit.

```
                    ┌─────────┐
Control Qubit: ───●──┤  Hadam  ├────────────────────────────●── [Measure]
                  │  └─────────┘                            │
                  │         ┌─────┐                 ┌─────┐ │
                  ├──[0]───┤ U_A ├──────[1]───────┤ U_B ├─┤
                  │         └─────┘                 └─────┘ │
Target Qubit:  ───┼─────────────────────────────────────────┼── [State |psi>]
                  │         ┌─────┐                 ┌─────┐ │
                  └──[1]───┤ U_B ├──────[0]───────┤ U_A ├─┘
                            └─────┘                 └─────┘
```

---

## 3. State-Transition Diagram for State Restoration (Rewinding)
The state transition flow for a target qubit being successfully rewound back to its past state $|\psi_0\rangle$.

```
         ┌───────────────────────────────┐
         │      Initial State |psi_0>    │
         └──────────────┬────────────────┘
                        │
                        │ Evolve under H for time t
                        ▼
         ┌───────────────────────────────┐
         │      Evolved State |psi(t)>   │
         └──────────────┬────────────────┘
                        │
                        │ Apply Restoration R(H, t)
                        ▼
         ┌───────────────────────────────┐
         │  Restored State |psi_rewound> │  <-- (Fidelity F = 1.0)
         └───────────────────────────────┘
```
