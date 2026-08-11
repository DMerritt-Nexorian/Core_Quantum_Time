# MACRO LIMITATION BOUNDS: SPECIAL RELATIVITY & SECOND LAW OF THERMODYNAMICS

## 1. Thermodynamic Bottleneck ($\Delta S_{\text{macro}} \ge 0$)
While individual quantum particles can undergo deterministic state rewinding (which is a unitary, reversible process), the Second Law of Thermodynamics must be strictly obeyed at the macroscopic scale.

### Macro-Scale Entropy Limit
For macroscopic multi-particle systems, the number of degrees of freedom is on the order of $N \sim 10^{23}$. Any attempt to reverse the states of all particles requires:
1. Reversing the thermal interactions with the external environment.
2. Perfect isolation of a macroscopic amount of matter.

According to statistical mechanics, the probability of spontaneous entropy reduction in a macroscopic system decreases exponentially with the particle count $N$:

$$P(\Delta S < 0) \propto e^{-\Delta S / k_B}$$

For $N > 10$, the system enters the classical-statistical domain, where macroscopic entropy $\Delta S_{\text{macro}}$ must satisfy:

$$\Delta S_{\text{macro}} \ge 0$$

The `core_quantum_time_engine` enforces a hard limit of $N \le 10$ quantum particles to maintain physical modeling safety and prevent macroscopic entropy leakage simulations that violate thermodynamic laws.

---

## 2. Relativistic Signal Bottleneck ($v_{\text{signal}} \le c$)
Quantum entanglement and superposition do not allow faster-than-light (FTL) classical information transmission.

### Special Relativity Guard
If a state restoration loop or negative dwell state could be used to transmit a bit value from a receiver to a sender instantaneously, it would violate special relativity and create causal paradoxes.

The signal velocity $v_{\text{signal}}$ must always satisfy:

$$v_{\text{signal}} \le c$$

where $c = 299,792,458 \text{ m/s}$ is the speed of light in vacuum.

The engine's `MacroGuard` validates all simulation parameters and automatically blocks any workflow where information transmission is detected to exceed $c$ or violate macroscopic thermodynamic constraints.
