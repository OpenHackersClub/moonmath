+++
title = "Maximum Modulus Principle"
description = "A non-constant holomorphic function on a connected open set cannot attain its maximum modulus at an interior point."
weight = 30
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "\\max_{z \\in \\overline{U}} |f(z)| = \\max_{z \\in \\partial U} |f(z)|"
prerequisites = ["cauchy-integral-formula"]
lean4_status = "complete"
+++

## Statement

Let $f$ be holomorphic on a connected open set $U$ and continuous on $\overline{U}$. If $f$ is not constant, then $|f|$ attains its maximum only on the boundary $\partial U$:

$$\max_{z \in \overline{U}} |f(z)| = \max_{z \in \partial U} |f(z)|$$

Equivalently: if $|f|$ has a local maximum at an interior point $c \in U$, then $f$ is constant near $c$ (and by the Identity Theorem, everywhere on $U$).

## Visualization

**$f(z) = z^2$ on the unit disk $|z| \le 1$.**

| $z$ | $|z|$ | $|f(z)| = |z^2|$ |
|-----|-------|-----------------|
| $0$ | $0$   | $0$             |
| $0.5$ | $0.5$ | $0.25$        |
| $i \cdot 0.7$ | $0.7$ | $0.49$ |
| $e^{i\pi/4}$ | $1$ | $1$      |
| $1$ | $1$   | $1$             |
| $-1$| $1$   | $1$             |

The maximum of $|z^2|$ on $|z| \le 1$ is $1$, attained on the boundary circle $|z| = 1$ — never at an interior point where $|z| < 1$.

```
  Boundary |z|=1: |f| = 1 everywhere (maximum)
  
       -i
       |
  -1---+---1    <-- boundary circle: |f| = 1
       |
       i
  
  Interior: |f(z)| = |z|^2 < 1 for |z| < 1
```

## Proof Sketch

1. Suppose $|f(c)| \ge |f(z)|$ for all $z$ in some disk $B(c, r) \subset U$.
2. The Cauchy integral formula gives $f(c) = \frac{1}{2\pi}\int_0^{2\pi} f(c + re^{i\theta})\,d\theta$ (mean value property).
3. Taking moduli: $|f(c)| \le \frac{1}{2\pi}\int_0^{2\pi}|f(c + re^{i\theta})|\,d\theta \le |f(c)|$.
4. Equality forces $|f(c + re^{i\theta})| = |f(c)|$ for almost every $\theta$ — a constant modulus on the circle.
5. A holomorphic function with constant modulus on any disk is itself constant (open mapping or CR equations). So $f$ is constant near $c$, hence everywhere by analytic continuation.

## Connections

The Maximum Modulus Principle gives [[Liouville's Theorem]] immediately (a bounded entire function is constant: take $U$ to be a large disk). It is closely related to the [[Cauchy Integral Formula]] via the mean-value property. For real-valued functions, the analogous result is the [[Mean Value Theorem]] for harmonic functions.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.AbsMax

open Complex

/-- **Maximum Modulus Principle**: on a preconnected open set, if |f| achieves
    its maximum at an interior point, then |f| is constant on the closure.
    Uses `Complex.norm_eqOn_of_isPreconnected_of_isMaxOn` from Mathlib. -/
theorem maximum_modulus_principle {E : Type*} [NormedAddCommGroup E] [NormedSpace ℂ E]
    {U : Set ℂ} {f : ℂ → E} {c : ℂ}
    (hc : IsPreconnected U) (hU : IsOpen U)
    (hf : DifferentiableOn ℂ f U)
    (hcU : c ∈ U)
    (hmax : IsMaxOn (‖f ·‖) U c) :
    ∀ x ∈ U, ‖f x‖ = ‖f c‖ :=
  fun x hx => norm_eqOn_of_isPreconnected_of_isMaxOn hc hU hf hcU hmax hx
```
