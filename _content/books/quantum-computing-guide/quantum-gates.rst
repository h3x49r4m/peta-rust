Quantum Bits and Quantum Gates
==============================

Single Qubit Gates
-----------------

Quantum gates are operations that manipulate qubits. Unlike classical gates, quantum gates are reversible and represented by unitary matrices.

Pauli Gates
~~~~~~~~~~~

1. **X Gate (NOT gate)**:
   $$X = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix}$$
   Flips $|0\rangle$ to $|1\rangle$ and vice versa.

2. **Y Gate**:
   $$Y = \begin{pmatrix} 0 & -i \\ i & 0 \end{pmatrix}$$

3. **Z Gate**:
   $$Z = \begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix}$$
   Adds a phase of $\pi$ to $|1\rangle$.

Hadamard Gate
~~~~~~~~~~~~

$$H = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & 1 \\ 1 & -1 \end{pmatrix}$$

Creates superposition:
- $H|0\rangle = \frac{1}{\sqrt{2}}(|0\rangle + |1\rangle)$
- $H|1\rangle = \frac{1}{\sqrt{2}}(|0\rangle - |1\rangle)$

Phase Gates
~~~~~~~~~~

1. **S Gate**:
   $$S = \begin{pmatrix} 1 & 0 \\ 0 & i \end{pmatrix}$$
   Phase gate with $\pi/2$ rotation.

2. **T Gate**:
   $$T = \begin{pmatrix} 1 & 0 \\ 0 & e^{i\pi/4} \end{pmatrix}$$
   Phase gate with $\pi/4$ rotation.

Multi-Qubit Gates
-----------------

CNOT Gate (Controlled-NOT)
~~~~~~~~~~~~~~~~~~~~~~~~~~

$$\text{CNOT} = \begin{pmatrix} 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0 \end{pmatrix}$$

Flips the target qubit if the control qubit is $|1\rangle$.

Controlled-U Gates
~~~~~~~~~~~~~~~~~

Any single-qubit gate U can be made controlled:

$$\text{C-U} = |0\rangle\langle0| \otimes I + |1\rangle\langle1| \otimes U$$

Universal Gate Sets
------------------

A set of gates is universal if any quantum operation can be approximated using them. Common universal sets:

1. **{H, T, CNOT}** - Clifford + T
2. **{H, S, CNOT}** - Clifford gates (need additional non-Clifford gate)
3. **{Toffoli gate}** - Universal for classical reversible computation

Quantum Circuits
---------------

Quantum algorithms are represented as quantum circuits:

1. **Wires** represent qubits
2. **Boxes** represent gates
3. **Time flows from left to right**

Example: Bell state preparation

.. image:: https://upload.wikimedia.org/wikipedia/commons/8/8a/Bell-state-circuit.svg
   :alt: Bell state circuit

This circuit creates the entangled Bell state: $\frac{1}{\sqrt{2}}(|00\rangle + |11\rangle)$