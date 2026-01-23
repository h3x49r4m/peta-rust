Quantum Algorithms
==================

Quantum algorithms leverage quantum phenomena to solve problems more efficiently than classical algorithms.

Deutsch-Jozsa Algorithm
------------------------

**Problem**: Determine if a function $f: \{0,1\}^n \to \{0,1\}$ is constant or balanced.

**Quantum Solution**: 
- Uses $n+1$ qubits
- Solves with 1 query (deterministic)
- Classical requires $2^{n-1}+1$ queries in worst case

**Circuit**:
1. Apply Hadamard to all qubits
2. Apply oracle
3. Apply Hadamard to first n qubits
4. Measure

Grover's Search Algorithm
--------------------------

**Problem**: Find marked item in unstructured database of N items.

**Quantum Advantage**: $O(\sqrt{N})$ vs $O(N)$ classical.

**Algorithm Steps**:
1. Initialize uniform superposition
2. Apply Grover operator $\sqrt{\pi N/4}$ times
3. Measure

Grover operator = Oracle + Diffusion operator

Shor's Factoring Algorithm
-------------------------

**Problem**: Factor integer N into its prime factors.

**Quantum Advantage**: Exponential speedup - best classical is sub-exponential.

**Key Components**:
1. **Quantum Fourier Transform (QFT)**
2. **Modular exponentiation**
3. **Period finding**

**Complexity**: $O((\log N)^3)$ quantum operations

Quantum Fourier Transform
--------------------------

QFT is the quantum analogue of discrete Fourier transform:

$$\text{QFT}|x\rangle = \frac{1}{\sqrt{N}}\sum_{y=0}^{N-1} e^{2\pi i xy/N}|y\rangle$$

Used in:
- Shor's algorithm
- Phase estimation
- Hidden subgroup problems

Variational Quantum Algorithms
------------------------------

Hybrid classical-quantum algorithms:

1. **VQE (Variational Quantum Eigensolver)**
   - Find ground state energy of molecules
   - Applications in chemistry and drug discovery

2. **QAOA (Quantum Approximate Optimization Algorithm)**
   - Solve combinatorial optimization problems
   - Applications in logistics and finance

Quantum Machine Learning
------------------------

1. **Quantum Support Vector Machine (QSVM)**
2. **Quantum Neural Networks (QNN)**
3. **Quantum Principal Component Analysis (QPCA)**

Algorithm Comparison
--------------------

| Algorithm | Problem | Classical Complexity | Quantum Complexity | Speedup |
|-----------|---------|---------------------|-------------------|---------|
| Deutsch-Jozsa | Oracle evaluation | $O(2^n)$ | $O(1)$ | Exponential |
| Grover | Search | $O(N)$ | $O(\sqrt{N})$ | Quadratic |
| Shor | Factoring | $O(e^{n^{1/3}})$ | $O(n^3)$ | Exponential |
| Simon | Period finding | $O(2^{n/2})$ | $O(n^3)$ | Exponential |