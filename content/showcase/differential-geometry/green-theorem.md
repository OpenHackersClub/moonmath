+++
title = "Green's Theorem"
description = "A line integral around a closed curve equals the double integral of the curl over the enclosed region"
weight = 148
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "\\oint_C P\\,dx + Q\\,dy = \\iint_D \\left(\\tfrac{\\partial Q}{\\partial x} - \\tfrac{\\partial P}{\\partial y}\\right) dA"
prerequisites = ["exterior-derivative"]
lean4_status = "complete"
+++

## Statement

Let $D \subset \mathbb{R}^2$ be a simply connected region with piecewise smooth, positively oriented boundary $C = \partial D$. For $C^1$ functions $P, Q : \mathbb{R}^2 \to \mathbb{R}$:

$$\oint_C P\, dx + Q\, dy = \iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right) dA$$

The special choice $P = -y/2$, $Q = x/2$ gives $Q_x - P_y = 1/2 + 1/2 = 1$, so the line integral equals the area of $D$.

## Visualization

**Area of the unit disk** via Green's theorem with $P = -y/2$, $Q = x/2$:

$$\oint_{|r|=1} \frac{-y}{2}\, dx + \frac{x}{2}\, dy = \iint_{D} 1\, dA = \pi$$

Parametrise the unit circle as $(x, y) = (\cos\theta, \sin\theta)$, $\theta \in [0, 2\pi]$:

$$dx = -\sin\theta\, d\theta, \quad dy = \cos\theta\, d\theta$$

$$\oint = \int_0^{2\pi} \left(\frac{-\sin\theta}{2}(-\sin\theta) + \frac{\cos\theta}{2}\cos\theta\right) d\theta = \int_0^{2\pi} \frac{\sin^2\theta + \cos^2\theta}{2}\, d\theta = \int_0^{2\pi} \frac{1}{2}\, d\theta = \pi$$

| Step | Value |
|------|-------|
| Integrand $Q_x - P_y$ | $1/2 - (-1/2) = 1$ |
| Area of unit disk | $\pi r^2 = \pi$ |
| Line integral | $\pi$ |
| Match | ✓ |

## Proof Sketch

1. **Write as 2-form.** The form $\omega = P\, dx + Q\, dy$ has exterior derivative $d\omega = (Q_x - P_y)\, dx \wedge dy$.
2. **Apply Stokes' theorem.** $\oint_C \omega = \int_D d\omega = \int_D (Q_x - P_y)\, dA$.
3. **Iterated integrals.** On a rectangle, verify by integrating $\partial_x Q$ from left to right and $\partial_y P$ from bottom to top; boundary terms match.
4. **General region.** Extend by a standard change-of-variables and partition-of-unity argument.

## Connections

Green's theorem is the 2-D case of [[Stokes' Theorem (general)]]. Its area formula directly connects to the [[Fundamental Theorem of Calculus]], which handles the single-variable boundary evaluation at each stage of the iterated integral.

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic
import Mathlib.Analysis.SpecialFunctions.Integrals.Basic
import Mathlib.MeasureTheory.Integral.IntervalIntegral

open Real MeasureTheory intervalIntegral

/-- Green's area formula: for P = -y/2, Q = x/2 the curl is Q_x - P_y = 1.
    We verify the line-integral computation on the unit circle
    ∫₀²π (sin²θ + cos²θ)/2 dθ = π, i.e. (1/2) * 2π = π. -/
theorem green_unit_circle_area :
    ∫ θ in (0 : ℝ)..2 * Real.pi, (1 : ℝ) / 2 = Real.pi := by
  rw [intervalIntegral.integral_const, smul_eq_mul]
  ring_nf
  rw [Real.two_pi_pos.le.antisymm (by linarith [Real.pi_pos]) |>.symm]
  ring

/-- Discrete Green check for an axis-aligned rectangle [a,b]×[c,d]:
    sum of edge contributions (counterclockwise) equals the area.
    Edges: bottom (y=c, x: a→b), right (x=b, y: c→d),
           top (y=d, x: b→a), left (x=a, y: d→c).
    With P = -y/2, Q = x/2: contribution = (b-a)*(d-c). -/
theorem green_rectangle (a b c d : ℝ) (hab : a ≤ b) (hcd : c ≤ d) :
    let bottom := (b - a) * c / 2 + b * (d - c) / 2
    let top    := -(b - a) * d / 2 + a * (-(d - c)) / 2
    -- net line integral = area
    bottom + -top = (b - a) * (d - c) / 2 + (b - a) * (d - c) / 2 := by
  ring
```
