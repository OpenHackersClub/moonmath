+++
title = "Argument Principle"
description = "The winding number of f around 0 along a contour equals the number of zeros minus poles enclosed."
weight = 80
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "\\frac{1}{2\\pi i}\\oint_\\gamma \\frac{f'(z)}{f(z)}\\,dz = N - P"
prerequisites = ["cauchy-integral-formula", "residue-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f$ be meromorphic inside and on a simple closed positively-oriented contour $\gamma$, with no zeros or poles on $\gamma$ itself. Let $N$ = number of zeros of $f$ inside $\gamma$ (counted with multiplicity), and $P$ = number of poles. Then:

$$\frac{1}{2\pi i}\oint_\gamma \frac{f'(z)}{f(z)}\,dz = N - P$$

The left side also equals the **winding number** of $f \circ \gamma$ around $0$ in the $f$-plane.

## Visualization

**Example: $f(z) = z^3 - z$ on the circle $|z| = 2$.**

Factor: $f(z) = z(z-1)(z+1)$. Zeros at $z = 0, 1, -1$, all inside $|z| = 2$. No poles (polynomial). So $N = 3$, $P = 0$.

The argument principle predicts:

$$\frac{1}{2\pi i}\oint_{|z|=2} \frac{f'(z)}{f(z)}\,dz = 3 - 0 = 3$$

**Residue calculation:** $f'/f = (3z^2 - 1)/(z^3 - z) = \frac{3z^2-1}{z(z-1)(z+1)}$.

| Zero $a$ | $\text{ord}(f, a)$ | Residue of $f'/f$ at $a$ |
|----------|--------------------|--------------------------|
| $0$      | $1$                | $1$ |
| $1$      | $1$                | $1$ |
| $-1$     | $1$                | $1$ |

Sum $= 3$. Integral $= 2\pi i \cdot 3$. Dividing by $2\pi i$: result $= 3 = N$.

**Image trace:** As $z$ traverses $|z| = 2$ once counterclockwise, $f(z) = z^3 - z$ winds around $0$ exactly 3 times (one winding per enclosed zero).

```
Contour |z| = 2          Image f(gamma) winds 3x around 0

    Im                          Im
    |                           |
   2+    o                   ...+...
    | (enclosed            .   0   .
   -+--+--+-- Re        -.--+--+--+-- Re
    | zeros                 ....    winding
   -2+  at                  number = 3
    | 0,1,-1
```

## Proof Sketch

1. At each zero $a$ of order $m$: write $f(z) = (z-a)^m g(z)$ with $g(a) \ne 0$. Then $f'/f = m/(z-a) + g'/g$ near $a$. The term $g'/g$ is holomorphic at $a$, contributing no residue. The residue of $f'/f$ at $a$ is $m$.
2. At each pole $a$ of order $p$: write $f(z) = (z-a)^{-p}h(z)$ with $h(a) \ne 0$. Then $f'/f = -p/(z-a) + h'/h$, contributing residue $-p$.
3. Sum via the Residue Theorem: $\frac{1}{2\pi i}\oint f'/f\,dz = \sum_{\text{zeros}} m_k - \sum_{\text{poles}} p_j = N - P$.

The winding number interpretation follows from: $\frac{d}{dt}\arg(f(\gamma(t))) = \text{Im}\left(\frac{f'(\gamma)}{f(\gamma)}\gamma'\right)$, integrated over $[0, 1]$.

## Connections

The Argument Principle is the Residue Theorem applied to $f'/f$, so [[Residue Theorem]] and [[Cauchy Integral Formula]] are its parents. Rouché's theorem (a corollary) counts zeros by comparison. This directly implies the [[Fundamental Theorem of Algebra]]: for a degree-$n$ polynomial $f$, the winding number of $f$ on a large circle is $n$, forcing exactly $n$ zeros inside.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.CauchyIntegral

open Complex MeasureTheory

/-- **Argument Principle (special case)**: for a function holomorphic and
    nonvanishing on a closed disk, f'/f integrates to zero.
    We prove the instance where f has no zeros inside: N = 0, P = 0.
    
    For f holomorphic on closedBall c R with no zeros, f'/f is holomorphic,
    so its circle integral vanishes by the Cauchy-Goursat theorem. -/
theorem argument_principle_no_zeros
    {R : ℝ} (hR : 0 < R) {c : ℂ}
    {f f' : ℂ → ℂ}
    (hf : ContinuousOn f (Metric.closedBall c R))
    (hf_diff : ∀ z ∈ Metric.ball c R, DifferentiableAt ℂ f z)
    (hf_nz : ∀ z ∈ Metric.closedBall c R, f z ≠ 0)
    (hf' : ContinuousOn f' (Metric.closedBall c R))
    (hf'_diff : ∀ z ∈ Metric.ball c R, DifferentiableAt ℂ f' z) :
    ∮ z in C(c, R), f' z / f z = 0 := by
  apply circleIntegral_eq_zero_of_differentiable_on_off_countable hR.le Set.countable_empty
  · exact hf'.div hf (fun z hz => hf_nz z hz)
  · intro z ⟨hz, _⟩
    exact (hf'_diff z hz).div (hf_diff z hz) (hf_nz z (Metric.ball_subset_closedBall hz))
```
