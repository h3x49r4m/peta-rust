---
title: "The Wave Function"
date: 2023-01-10
tags: ["physics", "quantum", "mathematics"]
snippet_id: "wave-function"
---

The quantum state of a particle is described by its wave function $\Psi(x,t)$, which contains all the information about the particle's quantum state.

Key properties of the wave function:

1. **Normalization**: $\int_{-\infty}^{\infty} |\Psi(x,t)|^2 dx = 1$
2. **Probability density**: $|\Psi(x,t)|^2$ gives the probability of finding the particle at position $x$ at time $t$
3. **Complex valued**: $\Psi(x,t)$ is generally a complex function

The wave function evolves according to the time-dependent Schrödinger equation:

$$i\hbar\frac{\partial}{\partial t}\Psi(x,t) = \hat{H}\Psi(x,t)$$

where $\hat{H}$ is the Hamiltonian operator representing the total energy of the system.