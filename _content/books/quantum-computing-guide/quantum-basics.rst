Quantum Mechanics Primer
=======================

Quantum States
--------------

In quantum mechanics, the state of a system is described by a wave function $\Psi$. For a qubit, this can be written as:

$$|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$$

where:
- $|0\rangle$ and $|1\rangle$ are the basis states
- $\alpha$ and $\beta$ are complex amplitudes
- $|\alpha|^2 + |\beta|^2 = 1$ (normalization condition)

Superposition
------------

Superposition allows a qubit to be in multiple states simultaneously. For example:

$$|\psi\rangle = \frac{1}{\sqrt{2}}|0\rangle + \frac{1}{\sqrt{2}}|1\rangle$$

This qubit has equal probability of being measured as 0 or 1.

Entanglement
------------

Entanglement is a quantum phenomenon where qubits become correlated in ways that classical bits cannot. For two entangled qubits:

$$|\Phi^+\rangle = \frac{1}{\sqrt{2}}(|00\rangle + |11\rangle)$$

Measuring one qubit instantly determines the state of the other, regardless of distance.

Measurement and Observables
--------------------------

When we measure a quantum system:
- The wave function collapses to one of the basis states
- The probability of each outcome is given by the square of its amplitude
- After measurement, the system remains in the measured state

Bloch Sphere Representation
---------------------------

A single qubit can be visualized on the Bloch sphere:

$$|\psi\rangle = \cos\left(\frac{\theta}{2}\right)|0\rangle + e^{i\phi}\sin\left(\frac{\theta}{2}\right)|1\rangle$$

where:
- $\theta$ is the polar angle (0 to $\pi$)
- $\phi$ is the azimuthal angle (0 to $2\pi$)