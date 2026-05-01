+++
title = "Morera's Theorem"
description = "A continuous function whose integral vanishes on every rectangle in a disk is holomorphic."
weight = 50
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "\\oint_{\\partial R} f(z)\\,dz = 0 \\;\\forall R \\implies f \\text{ holomorphic}"
prerequisites = ["cauchy-integral-formula"]
lean4_status = "complete"
+++

## Statement

**Morera's Theorem** is the converse of Cauchy's theorem: Let $f$ be a continuous function on an open disk $B(c, r)$. If

$$\int_{\partial R} f(z)\,dz = 0$$

for every closed rectangle $R$ whose interior is contained in $B(c, r)$, then $f$ is holomorphic on $B(c, r)$.

Equivalently: a continuous function that is **conservative** (zero integral on rectangles) is automatically differentiable.

## Visualization

**Verify for $f(z) = z$ on any rectangle.**

Take the rectangle with corners $0, 1, 1+i, i$ (traversed counterclockwise):

```
  i -------- 1+i
  |            |
  |            |
  0 -------- 1
```

Parameterize each side:

| Side | Path | Integral |
|------|------|---------|
| Bottom: $0 \to 1$ | $z = t$, $dz = dt$ | $\int_0^1 t\,dt = 1/2$ |
| Right: $1 \to 1+i$ | $z = 1+it$, $dz = i\,dt$ | $\int_0^1 (1+it)\,i\,dt = i - 1/2$ |
| Top: $1+i \to i$ | $z = (1-t)+i$, $dz = -dt$ | $\int_0^1 ((1-t)+i)(-dt) = -1/2 + i(-1)... $ |
| Left: $i \to 0$ | $z = i(1-t)$, $dz = -i\,dt$ | $\int_0^1 i(1-t)(-i)\,dt = 1/2 - 1/2$ |

Summing: $1/2 + (i - 1/2) + (-1/2 - i) + (-1/2 + 1/2) = 0$. The total vanishes, consistent with $f(z) = z$ being holomorphic.

A non-holomorphic function $f(z) = \bar{z}$ gives $\oint \bar{z}\,dz = -2i \cdot \text{Area}(R) \ne 0$.

## Proof Sketch

1. Define $F(z) = \int_c^z f(\zeta)\,d\zeta$ along a rectilinear path (horizontal then vertical) from $c$ to $z$.
2. The vanishing rectangle integrals ensure $F$ is well-defined (path-independent within the disk).
3. Compute $F'(z) = f(z)$ directly: the difference quotient $[F(z+h) - F(z)]/h \to f(z)$ by continuity of $f$.
4. So $F$ is holomorphic, and the derivative of a holomorphic function is holomorphic, making $f = F'$ holomorphic.

## Connections

Morera's theorem pairs with the [[Cauchy Integral Formula]] as its converse: Cauchy says holomorphic $\Rightarrow$ zero integrals; Morera says zero integrals $\Rightarrow$ holomorphic. It is also used in [[Liouville's Theorem]] proofs via normal families: a locally uniform limit of holomorphic functions passes the Morera test and hence is holomorphic.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.HasPrimitives

open Complex

/-- **Morera's Theorem**: a continuous function on a disk whose integrals on
    rectangles all vanish is holomorphic (has a primitive, hence is differentiable).
    Uses `Complex.IsConservativeOn.isExactOn_ball` from Mathlib. -/
theorem morera_theorem {c : ℂ} {r : ℝ} {f : ℂ → ℂ}
    (hcont : ContinuousOn f (Metric.ball c r))
    (hcons : IsConservativeOn f (Metric.ball c r)) :
    ∃ F : ℂ → ℂ, ∀ z ∈ Metric.ball c r, HasDerivAt F (f z) z :=
  (hcons.isExactOn_ball hcont).hasDerivAt
```
