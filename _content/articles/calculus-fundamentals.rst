---
title: "Calculus Fundamentals"
date: 2023-02-20
tags: ["mathematics", "calculus", "analysis"]
author: "Prof. John Doe"
---


Calculus is the mathematical study of continuous change. It has two major branches: differential calculus and integral calculus.


Derivatives
-----------

.. snippet-card:: derivatives

The derivative of a function $f(x)$ with respect to $x$ is defined as:

$$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

This represents the rate of change of the function at any given point.

Common derivatives include:

- $\frac{d}{dx}(x^n) = nx^{n-1}$
- $\frac{d}{dx}(\sin x) = \cos x$
- $\frac{d}{dx}(e^x) = e^x$


Integrals
---------

.. snippet-card:: integrals

The integral is the inverse operation of differentiation. The definite integral is defined as:

$$\int_a^b f(x) dx = \lim_{n \to \infty} \sum_{i=1}^{n} f(x_i^*) \Delta x$$

This represents the area under the curve $f(x)$ from $x = a$ to $x = b$.

Fundamental Theorem of Calculus
--------------------------------

The Fundamental Theorem of Calculus connects differentiation and integration:

$$\frac{d}{dx}\int_a^x f(t) dt = f(x)$$

This theorem shows that differentiation and integration are inverse operations.
