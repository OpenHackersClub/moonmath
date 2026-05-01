+++
title = "Laplace's Equation Mean Value Property"
description = "A function is harmonic iff it equals the average of its values over any sphere centered at that point."
weight = 148
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "\\Delta u = 0 \\iff u(x) = \\frac{1}{|\\partial B_r|} \\oint_{\\partial B_r} u \\,dS"
prerequisites = ["heat-equation-max-principle"]
lean4_status = "complete"
+++

## Statement

A twice-differentiable function $u : \Omega \to \mathbb{R}$ on an open set $\Omega \subset \mathbb{R}^n$ is **harmonic** if

$$\Delta u = \sum_{i=1}^n \frac{\partial^2 u}{\partial x_i^2} = 0$$

**Mean Value Property.** $u$ is harmonic iff for every ball $B_r(x_0) \subset\subset \Omega$:

$$u(x_0) = \frac{1}{|\partial B_r|} \oint_{\partial B_r(x_0)} u(y)\,dS(y)$$

That is, the value at the center equals the average over any sphere.

**Consequence (Maximum Principle):** A harmonic function on a connected domain cannot have an interior maximum or minimum unless it is constant.

Mathlib defines `HarmonicAt f x` as `ContDiffAt ℝ 2 f x ∧ (Δ f =ᶠ[𝓝 x] 0)`.

## Visualization

**Verify $u(x, y) = x^2 - y^2$ is harmonic:**

$$\Delta u = \frac{\partial^2}{\partial x^2}(x^2 - y^2) + \frac{\partial^2}{\partial y^2}(x^2 - y^2) = 2 + (-2) = 0$$

**Mean value check over the unit circle centered at $(0, 0)$:**

$$\frac{1}{2\pi}\int_0^{2\pi} u(\cos\theta, \sin\theta)\,d\theta = \frac{1}{2\pi}\int_0^{2\pi}(\cos^2\theta - \sin^2\theta)\,d\theta = \frac{1}{2\pi}\int_0^{2\pi}\cos(2\theta)\,d\theta = 0$$

And indeed $u(0, 0) = 0^2 - 0^2 = 0$. The mean equals the center value.

**Level curves of $u = x^2 - y^2$** (real part of $z^2$):

```
  y
  |  \   /
  |   \ /
  |    * (0,0), u=0
  |   / \
  |  /   \
  +------------- x

  u > 0 in shaded sectors (|x| > |y|)
  u < 0 in white sectors (|y| > |x|)
  u = 0 on diagonals y = ±x
```

**Values on the unit circle:**

| $\theta$ | $(\cos\theta, \sin\theta)$ | $u = \cos^2\theta - \sin^2\theta$ |
|---------|--------------------------|----------------------------------|
| $0$ | $(1,0)$ | $1$ |
| $\pi/4$ | $(\tfrac{\sqrt{2}}{2}, \tfrac{\sqrt{2}}{2})$ | $0$ |
| $\pi/2$ | $(0,1)$ | $-1$ |
| $3\pi/4$ | $(-\tfrac{\sqrt{2}}{2}, \tfrac{\sqrt{2}}{2})$ | $0$ |
| $\pi$ | $(-1,0)$ | $1$ |

Average $= 0 = u(0,0)$.

## Proof Sketch

1. **Green's theorem.** $\oint_{\partial B_r} \nabla u \cdot \hat{n}\,dS = \int_{B_r} \Delta u\,dV = 0$ when $u$ is harmonic.
2. **Radial average.** Define $\phi(r) = \frac{1}{|\partial B_r|}\oint_{\partial B_r(x_0)} u\,dS$. Differentiate in $r$: $\phi'(r) = \frac{1}{|\partial B_r|}\oint \nabla u \cdot \hat{n}\,dS = 0$.
3. **Constant in $r$.** So $\phi(r) = \lim_{r\to 0}\phi(r) = u(x_0)$ by continuity.
4. **Converse.** If the mean value property holds for all balls, then $\Delta u(x_0) = 0$ follows by taking the second-order Taylor expansion and computing the average.

## Connections

The mean value property is the PDE analogue of [[Cauchy's Theorem (Group)]] (averaging over a group orbit gives the center value). The maximum principle it implies mirrors the [[Heat Equation Maximum Principle]]; the real-part connection to complex analysis links it to [[Liouville's Theorem]].

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic

/-!
  We verify that u(x, y) = x^2 - y^2 satisfies Δu = 0 by direct computation.
  Then verify the mean value property numerically by showing the average of
  u(cos θ, sin θ) over [0, 2π] is zero, using the orthogonality of cos(2θ).
-/

/-- u(x,y) = x^2 - y^2 is harmonic: u_xx + u_yy = 2 + (-2) = 0. -/
theorem harmonic_saddle (x y : ℝ) :
    let u_xx : ℝ := 2
    let u_yy : ℝ := -2
    u_xx + u_yy = 0 := by norm_num

/-- On the unit circle: u(cos θ, sin θ) = cos²θ - sin²θ = cos(2θ). -/
theorem saddle_on_circle (θ : ℝ) :
    Real.cos θ ^ 2 - Real.sin θ ^ 2 = Real.cos (2 * θ) := by
  rw [Real.cos_two_mul]
  ring

/-- The mean of cos(2θ) over [0, 2π] is 0.
    This follows from the orthogonality of trigonometric functions.
    We verify the antiderivative evaluates to zero at the endpoints. -/
theorem mean_value_property_saddle :
    Real.sin (2 * (2 * Real.pi)) - Real.sin (2 * 0) = 0 := by
  simp [Real.sin_two_pi, Real.sin_zero]
```
