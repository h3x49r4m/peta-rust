Quantum Error Correction
========================

Quantum systems are fragile and susceptible to errors. Quantum error correction (QEC) is essential for building reliable quantum computers.

Types of Quantum Errors
-----------------------

1. **Bit flip errors**: $|0\rangle \leftrightarrow |1\rangle$
   - Caused by X operator
   - Classical bit flip analogue

2. **Phase flip errors**: $|0\rangle \to |0\rangle$, $|1\rangle \to -|1\rangle$
   - Caused by Z operator
   - Purely quantum phenomenon

3. **Bit-phase flip errors**: Combination of bit and phase flip
   - Caused by Y operator
   - Most general single-qubit error

No-Cloning Theorem
-----------------

Quantum states cannot be copied:
- Cannot create identical copy of unknown quantum state
- Prevents simple repetition coding used in classical error correction

Error Correction Principles
--------------------------

1. **Redundancy**: Encode logical qubits using multiple physical qubits
2. **Syndrome measurement**: Detect errors without collapsing the state
3. **Recovery**: Apply correction based on syndrome

Three-Qubit Bit Flip Code
-------------------------

Encodes one logical qubit using three physical qubits:

$$|0_L\rangle = |000\rangle$$
$$|1_L\rangle = |111\rangle$$

Can correct single bit flip errors.

**Syndrome measurement**:
- Measure parity of qubits 1&2 and 2&3
- Identifies which qubit flipped

Nine-Qubit Shor Code
--------------------

Corrects arbitrary single-qubit errors:

$$|0_L\rangle = \frac{1}{2\sqrt{2}}(|000\rangle + |111\rangle) \otimes (|000\rangle + |111\rangle) \otimes (|000\rangle + |111\rangle)$$
$$|1_L\rangle = \frac{1}{2\sqrt{2}}(|000\rangle - |111\rangle) \otimes (|000\rangle - |111\rangle) \otimes (|000\rangle - |111\rangle)$$

Uses 9 physical qubits for 1 logical qubit.

Stabilizer Codes
----------------

Efficient framework for QEC:
- Define stabilizer group
- Logical states are +1 eigenstates of all stabilizers
- Error syndromes from stabilizer measurements

Surface Codes
-------------

Leading approach for fault-tolerant quantum computing:

- 2D arrangement of qubits on lattice
- Local stabilizer measurements
- High threshold (~1%)
- Scalable architecture

Fault-Tolerant Quantum Computing
--------------------------------

Requirements:
1. **Error correction**: Below threshold error rates
2. **Fault-tolerant gates**: Operations that don't spread errors
3. **Magic state distillation**: Create high-fidelity resource states

Threshold Theorem
-----------------

If physical error rate is below threshold:
- Arbitrary long quantum computations possible
- Overhead scales polylogarithmically with computation size

Current Challenges
------------------

1. **High overhead**: Requires many physical qubits per logical qubit
2. **Connectivity constraints**: Limited qubit interactions
3. **Measurement errors**: Imperfect syndrome extraction
4. **Correlated errors**: Non-independent error models