---
title: RST Math Syntax Test
date: 2026-02-06
type: article
tags: [rst, math, test]
---

This article tests the new official RST math syntax support.

## Official RST Math Syntax

### Display Math with 

Here's a simple equation using the official RST math directive:

.. math::
   :label: equation1
   
   E = mc^2

Another example with more complex formatting:

.. math::
   :label: quadratic
   
   x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}

### Labeled Equations

The equation above has a label "quadratic" that can be referenced.

Here's another labeled equation:

.. math::
   :label: pythagorean
   
   a^2 + b^2 = c^2

### Inline Math with :math: Role

You can use inline math like this: The mass-energy equivalence is :math:`E = mc^2`.

More inline examples:

- The area of a circle is :math:`A = \pi r^2`
- The derivative of :math:`f(x) = x^2` is :math:`f'(x) = 2x`
- Euler's identity: :math:`e^{i\pi} + 1 = 0`

### Complex Display Math

A more complex equation with matrices:

.. math::
   
   \begin{pmatrix}
   a & b \\
   c & d
   \end{pmatrix}
   \begin{pmatrix}
   x \\
   y
   \end{pmatrix}
   =
   \begin{pmatrix}
   ax + by \\
   cx + dy
   \end{pmatrix}

Integral example:

.. math::
   :label: gaussian
   
   \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}

## Legacy Syntax Support

The legacy LaTeX-style syntax still works for backward compatibility:

Display math with double dollar signs:

$$ \sum_{i=1}^{n} i = \frac{n(n+1)}{2} $$

Inline math with single dollar signs: The value of $x$ is 42.

## Mixed Usage

You can mix both syntaxes in the same document. Here's an official RST directive:

.. math::
   
   \nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}

And here's legacy syntax: $\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$

Both render correctly!
