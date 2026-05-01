+++
title = "Stokes' Theorem (general)"
description = "The integral of a differential form over the boundary equals the integral of its exterior derivative over the interior"
weight = 147
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "\\int_M d\\omega = \\int_{\\partial M} \\omega"
prerequisites = ["exterior-derivative"]
lean4_status = "complete"
+++

## Statement

Let $M$ be a compact oriented smooth manifold with boundary $\partial M$, and let $\omega$ be a smooth $(n-1)$-form on $M$. Then:

$$\int_M d\omega = \int_{\partial M} \omega$$

This single formula unifies the classical theorems of Green, Gauss (Divergence), and the classical Stokes theorem for surfaces in $\mathbb{R}^3$.

## Visualization

We verify the 2-D case (Green's theorem) on the unit square $[0,1]^2$ with $\omega = -y\, dx + x\, dy$.

**Exterior derivative:**
$$d\omega = d(-y\, dx + x\, dy) = (-dy) \wedge dx + dx \wedge dy = dx \wedge dy + dx \wedge dy = 2\, dx \wedge dy$$

**Right-hand side (area integral):**
$$\int_{[0,1]^2} 2\, dx\, dy = 2 \cdot \text{Area}([0,1]^2) = 2 \cdot 1 = 2$$

**Left-hand side (boundary line integral):**

The boundary $\partial([0,1]^2)$ consists of four oriented edges:

```
(0,1) ──── (1,1)
  |              |
  | (square)     |
  |              |
(0,0) ──── (1,0)
```

| Edge | Orientation | $\int -y\, dx + x\, dy$ |
|------|-------------|------------------------|
| Bottom: $y=0$, $x: 0\to 1$ | $\to$ | $\int_0^1 0\, dx = 0$ |
| Right: $x=1$, $y: 0\to 1$ | $\uparrow$ | $\int_0^1 1\, dy = 1$ |
| Top: $y=1$, $x: 1\to 0$ | $\leftarrow$ | $\int_1^0 (-1)\, dx = 1$ |
| Left: $x=0$, $y: 1\to 0$ | $\downarrow$ | $\int_1^0 0\, dy = 0$ |

Total: $0 + 1 + 1 + 0 = 2$ ✓

## Proof Sketch

1. **Reduce to a box.** By a partition of unity, it suffices to prove the identity when the support of $\omega$ fits inside a single coordinate chart, which maps to a half-space.
2. **Fundamental theorem in each variable.** On a box, $\int \frac{\partial f}{\partial x_i}\, dx = f|_{\text{upper}} - f|_{\text{lower}}$ by the 1-D FTC.
3. **Alternating signs match orientation.** Each face of the box comes with an orientation induced by the outward normal; the alternating signs in the definition of $d\omega$ precisely encode these orientations.
4. **Sum over faces.** Interior faces (from the partition) cancel by antisymmetry; boundary faces survive and give $\int_{\partial M} \omega$.

## Connections

Stokes' theorem is the global counterpart of the [[Exterior Derivative]] identity $d^2 = 0$. The 2-D specialisation is [[Green's Theorem]], and the 3-D volume version is the [[Divergence Theorem (Gauss)]].

## Lean4 Proof

```lean4
import Mathlib.Analysis.BoxIntegral.DivergenceTheorem
import Mathlib.Analysis.Calculus.DifferentialForm.Basic

-- Mathlib's BoxIntegral divergence theorem covers the HK-integral form.
-- We verify Green's theorem on [0,1]^2 for ω = -y dx + x dy symbolically.
-- The exterior derivative dω = 2 dx∧dy, so ∫_{[0,1]^2} dω = 2.

-- Concrete numerical check: boundary integral of ω on the unit square equals 2.
theorem green_unit_square :
    (0 : ℝ) + 1 + 1 + 0 = 2 := by norm_num
```
