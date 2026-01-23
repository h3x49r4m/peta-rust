---
title: "Integrals"
date: 2023-02-16
tags: ["mathematics", "calculus", "integrals"]
snippet_id: "integrals"
---

The integral is the inverse operation of differentiation, representing the accumulation of quantities.

Definition
----------

The definite integral of a function $f(x)$ from $a$ to $b$ is defined as:

$$\int_a^b f(x) dx = \lim_{n \to \infty} \sum_{i=1}^{n} f(x_i^*) \Delta x$$

This represents the area under the curve $f(x)$ from $x = a$ to $x = b$.

Fundamental Theorem of Calculus
--------------------------------

The Fundamental Theorem of Calculus connects differentiation and integration:

$$\frac{d}{dx}\int_a^x f(t) dt = f(x)$$

This theorem shows that differentiation and integration are inverse operations.

Common Integrals
----------------

Here are some common integrals:

1. **Power rule**: $\int x^n dx = \frac{x^{n+1}}{n+1} + C$ (for $n \neq -1$)
2. **Exponential**: $\int e^x dx = e^x + C$
3. **Trigonometric**:
   - $\int \sin x dx = -\cos x + C$
   - $\int \cos x dx = \sin x + C$
   - $\int \sec^2 x dx = \tan x + C$

Applications
------------

Integrals have numerous applications in:
- Calculating areas and volumes
- Physics (work, energy)
- Probability (probability distributions)
- Economics (total cost, total revenue)