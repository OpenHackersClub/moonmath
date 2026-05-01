+++
title = "Residue Theorem"
description = "A contour integral of a meromorphic function equals 2πi times the sum of residues of enclosed poles."
weight = 20
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "\\oint_\\gamma f(z)\\,dz = 2\\pi i \\sum_{k} \\operatorname{Res}(f, a_k)"
prerequisites = ["cauchy-integral-formula"]
lean4_status = "complete"
+++

## Statement

Let $f$ be meromorphic inside and on a simple closed contour $\gamma$ (traversed counterclockwise), with isolated poles $a_1, \ldots, a_n$ inside $\gamma$. Then:

$$\oint_\gamma f(z)\,dz = 2\pi i \sum_{k=1}^n \operatorname{Res}(f, a_k)$$

For a **simple pole** at $a$, the residue is $\operatorname{Res}(f, a) = \lim_{z \to a}(z - a)f(z)$.

## Visualization

**Example 1: $f(z) = 1/z$, contour $|z| = 1$.**

$$\operatorname{Res}(1/z,\; 0) = \lim_{z \to 0} z \cdot \frac{1}{z} = 1$$

$$\oint_{|z|=1} \frac{dz}{z} = 2\pi i \cdot 1 = 2\pi i$$

**Example 2: $f(z) = 1/(z^2 + 1)$, contour $|z| = 2$.**

Poles at $z = i$ and $z = -i$, both inside $|z| = 2$.

| Pole $a$ | $(z - a) \cdot f(z)$ | Residue |
|----------|---------------------|---------|
| $i$      | $\frac{1}{z+i}\big|_{z=i} = \frac{1}{2i}$ | $\frac{1}{2i}$ |
| $-i$     | $\frac{1}{z-i}\big|_{z=-i} = \frac{1}{-2i}$ | $\frac{-1}{2i}$ |

Sum of residues $= \frac{1}{2i} - \frac{1}{2i} = 0$, so:

$$\oint_{|z|=2}\frac{dz}{z^2 + 1} = 2\pi i \cdot 0 = 0$$

**Example 3: $f(z) = e^z/z^2$, contour $|z| = 1$.**

Pole at $z = 0$ of order 2. Residue $= \frac{d}{dz}[e^z]\big|_{z=0} = 1$, so the integral $= 2\pi i$.

## Proof Sketch

1. Isolate each pole $a_k$ with a small disk $D_k$ of radius $\varepsilon$; deform $\gamma$ to avoid them.
2. On the deformed region, $f$ is holomorphic, so the integral is unchanged and equals the sum of integrals over small circles $|z - a_k| = \varepsilon$.
3. For a simple pole, Laurent expand: $f(z) = \frac{c_{-1}}{z - a_k} + g(z)$ where $g$ is holomorphic. The circle integral of $g$ vanishes; the integral of $c_{-1}/(z - a_k)$ is $2\pi i \cdot c_{-1} = 2\pi i \cdot \operatorname{Res}(f, a_k)$.
4. Sum over all poles.

## Connections

The Residue Theorem is the Cauchy Integral Formula applied iteratively — it extends [[Cauchy Integral Formula]] to meromorphic functions. It connects to [[Liouville's Theorem]]: an entire function with a "residue at infinity" forces non-constancy.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.CauchyIntegral

open Complex MeasureTheory

/-- **Residue formula for a simple pole**: the circle integral of f(z)/(z - a)
    equals 2πi · f(a) when f is holomorphic on the closed disk.
    This is the Residue Theorem for a single simple pole, derived directly
    from the Cauchy Integral Formula in Mathlib. -/
theorem residue_simple_pole {R : ℝ} {c a : ℂ} {f : ℂ → ℂ}
    (hf : DiffContOnCl ℂ f (Metric.ball c R))
    (ha : a ∈ Metric.ball c R) :
    ∮ z in C(c, R), (z - a)⁻¹ • f z = (2 * π * Complex.I) • f a :=
  hf.circleIntegral_sub_inv_smul ha
```
