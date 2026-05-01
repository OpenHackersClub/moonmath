+++
title = "Cauchy Integral Formula"
description = "A holomorphic function's value at any interior point is determined by its values on the boundary circle."
weight = 10
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "f(w) = \\frac{1}{2\\pi i}\\oint_{|z-c|=R} \\frac{f(z)}{z - w}\\,dz"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $f$ be holomorphic on a closed disk $\overline{B}(c, R)$ and let $w \in B(c, R)$ be any interior point. Then:

$$f(w) = \frac{1}{2\pi i}\oint_{|z - c| = R} \frac{f(z)}{z - w}\,dz$$

Equivalently, the smeared form:

$$\frac{1}{2\pi i}\oint \frac{f(z)}{z - w}\,dz = f(w)$$

holds whenever $f$ is holomorphic on the open disk and continuous on the closure.

## Visualization

Take $f(z) = 1$ (constant), $c = 0$, $R = 1$, $w = 0$. The formula predicts:

$$\frac{1}{2\pi i}\oint_{|z|=1}\frac{1}{z}\,dz = 1$$

Parameterize: $z = e^{i\theta}$, $dz = ie^{i\theta}d\theta$.

```
theta:    0       pi/2     pi      3pi/2    2pi
z:        1        i      -1       -i        1
1/z:      1       -i      -1        i        1
```

$$\oint_{|z|=1}\frac{dz}{z} = \int_0^{2\pi}\frac{ie^{i\theta}}{e^{i\theta}}\,d\theta = \int_0^{2\pi}i\,d\theta = 2\pi i$$

Dividing by $2\pi i$ gives $f(0) = 1$. Confirmed.

For $f(z) = z^n$ with $n \ge 1$ and $w = 0$: the integrand $z^n/z = z^{n-1}$ has no residue at $0$ when $n \ge 1$, so the integral is $0 = f(0)$ only when $f(0) = 0$. For $w \ne 0$ inside the disk, the formula reconstructs $w^n$ from boundary values.

## Proof Sketch

1. For $w$ outside the contour, $z \mapsto f(z)/(z-w)$ is holomorphic everywhere inside, so the integral vanishes by Cauchy's theorem.
2. For $w$ inside, write $\frac{f(z)}{z-w} = \frac{f(z) - f(w)}{z-w} + \frac{f(w)}{z-w}$.
3. The first term extends continuously to $w$ (holomorphicity of $f$ at $w$), so its integral over a small circle around $w$ tends to $0$.
4. The second term contributes $f(w) \cdot \oint \frac{dz}{z-w} = f(w) \cdot 2\pi i$.
5. Deformation of contour (the integral is unchanged when deforming through regions where $f$ is holomorphic) finishes the proof.

## Connections

The formula is the engine behind [[Liouville's Theorem]] (bounded entire functions are constant) and the [[Fundamental Theorem of Algebra]] (non-constant polynomials have roots). It also drives [[Taylor's Theorem]] for complex functions: differentiating under the integral sign yields power series coefficients.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.CauchyIntegral

open Complex MeasureTheory

/-- **Cauchy Integral Formula**: for `f` holomorphic on a closed disk,
    the value at any interior point `w` is recovered from the circle integral.
    Uses `DiffContOnCl.circleIntegral_sub_inv_smul` from Mathlib. -/
theorem cauchy_integral_formula {R : ℝ} {c w : ℂ} {f : ℂ → ℂ}
    (hf : DiffContOnCl ℂ f (Metric.ball c R))
    (hw : w ∈ Metric.ball c R) :
    ((2 * π * Complex.I)⁻¹ • ∮ z in C(c, R), (z - w)⁻¹ • f z) = f w :=
  hf.two_pi_i_inv_smul_circleIntegral_sub_inv_smul hw
```
