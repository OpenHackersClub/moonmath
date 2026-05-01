+++
title = "Wronskian"
description = "The Wronskian W(f,g) = fg' - f'g detects linear independence of two solutions to a second-order ODE."
weight = 144
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "W(f,g) = fg'-f'g \\neq 0 \\iff \\{f,g\\}\\text{ linearly independent}"
prerequisites = ["integrating-factor"]
lean4_status = "complete"
+++

## Statement

Given two differentiable functions $f, g$, their **Wronskian** is

$$W(f, g)(x) = f(x)g'(x) - f'(x)g(x) = \det\begin{pmatrix} f & g \\ f' & g' \end{pmatrix}$$

**Linear independence criterion:** If $f$ and $g$ are solutions to the same second-order linear ODE on an interval, then either $W(f,g)(x) \neq 0$ for all $x$ in the interval (the solutions are linearly independent) or $W(f,g)(x) = 0$ for all $x$ (the solutions are proportional).

This gives a decisive, computable test: two solutions form a **fundamental system** (basis for all solutions) iff $W \neq 0$ at any single point.

For polynomials over a ring, Mathlib defines `Polynomial.wronskian a b = a * b.derivative - a.derivative * b`, and the key identity `wronskian_self_eq_zero` shows $W(a,a) = 0$.

## Visualization

**Standard example: $f = \sin x$, $g = \cos x$** (both solve $y'' + y = 0$):

$$W(\sin, \cos)(x) = \sin x \cdot (-\sin x) - \cos x \cdot \cos x = -\sin^2 x - \cos^2 x = -1$$

Since $W = -1 \neq 0$ everywhere, $\{\sin, \cos\}$ is linearly independent.

| $x$ | $f = \sin x$ | $g = \cos x$ | $f' = \cos x$ | $g' = -\sin x$ | $W = fg'-f'g$ |
|-----|-------------|-------------|--------------|----------------|--------------|
| $0$ | $0$ | $1$ | $1$ | $0$ | $0-1 = -1$ |
| $\pi/4$ | $\tfrac{\sqrt{2}}{2}$ | $\tfrac{\sqrt{2}}{2}$ | $\tfrac{\sqrt{2}}{2}$ | $-\tfrac{\sqrt{2}}{2}$ | $-1$ |
| $\pi/2$ | $1$ | $0$ | $0$ | $-1$ | $-1-0 = -1$ |
| $\pi$ | $0$ | $-1$ | $-1$ | $0$ | $0-1 = -1$ |

$W \equiv -1$ confirms the functions are independent at every point.

**Dependent example:** $f = e^x$, $g = 2e^x$.

$$W(e^x, 2e^x) = e^x \cdot 2e^x - e^x \cdot 2e^x = 0$$

$W \equiv 0$ reflects that $g = 2f$ — the functions are proportional.

## Proof Sketch

1. **Abel's identity.** For $y'' + p(x)y' + q(x)y = 0$, differentiate $W = fg' - f'g$ and use the ODE to show $W' = -p(x)W$.
2. **Integrating factor.** Thus $W(x) = W(x_0)\exp\!\left(-\int_{x_0}^x p(s)\,ds\right)$.
3. **Dichotomy.** Since the exponential is never zero, $W(x_0) = 0$ iff $W \equiv 0$ on the interval.
4. **Basis.** If $W(x_0) \neq 0$ at some $x_0$, then for any solution $y$, the system $c_1 f(x_0) + c_2 g(x_0) = y(x_0)$, $c_1 f'(x_0) + c_2 g'(x_0) = y'(x_0)$ has a unique solution (Cramer's rule, det = $W(x_0)$), giving $y = c_1 f + c_2 g$ by uniqueness of IVPs.

## Connections

The Wronskian determinant is a $2 \times 2$ special case of [[Determinant Multiplicativity]] and Cramer's rule ([[Cramer's Rule]]). The ODE uniqueness step appeals to [[Picard–Lindelöf Theorem]].

## Lean4 Proof

```lean4
import Mathlib.RingTheory.Polynomial.Wronskian

/-!
  Mathlib defines `Polynomial.wronskian a b = a * b.derivative - a.derivative * b`.
  We verify the key properties:
    1. W(a, a) = 0 for any polynomial (wronskian_self_eq_zero)
    2. W is antisymmetric: -W(a,b) = W(b,a) (wronskian_neg_eq)
-/

open Polynomial in
/-- The Wronskian of any polynomial with itself is zero. -/
example (a : ℤ[X]) : wronskian a a = 0 :=
  wronskian_self_eq_zero a

open Polynomial in
/-- Wronskian is anti-symmetric: W(b, a) = -W(a, b). -/
example (a b : ℤ[X]) : wronskian b a = -wronskian a b := by
  rw [← wronskian_neg_eq]

/-- Concrete: W(X, X^2) = X^2 in ℤ[X].
    X * (X^2)' - X' * X^2 = X * 2X - 1 * X^2 = 2X^2 - X^2 = X^2. -/
example : Polynomial.wronskian (Polynomial.X : ℤ[X]) (Polynomial.X ^ 2) =
    Polynomial.X ^ 2 := by
  simp [Polynomial.wronskian, Polynomial.derivative_pow, Polynomial.derivative_X]
  ring
```
