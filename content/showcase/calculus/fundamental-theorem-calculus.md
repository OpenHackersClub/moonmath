+++
title = "Fundamental Theorem of Calculus"
description = "Integration and differentiation are inverse operations"
weight = 30
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\int_a^b f'(x)\\,dx = f(b) - f(a)"
prerequisites = ["mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

**Part I (Antiderivative):** If $f$ is continuous on $[a, b]$ and $F(x) = \int_a^x f(t)\,dt$, then $F$ is differentiable on $(a, b)$ with $F'(x) = f(x)$.

**Part II (Evaluation):** If $f'$ is integrable on $[a, b]$ and $f$ has derivative $f'$ at every point in $(a, b)$, then:

$$\int_a^b f'(x)\,dx = f(b) - f(a)$$

This is the central theorem connecting the two branches of calculus.

## Visualization

Approximate $\int_1^3 2x\,dx$ (exact answer: $f(3)-f(1) = 9-1 = 8$) using the trapezoid rule with $n = 4$ strips of width $\Delta x = 0.5$:

| Strip $[x_i, x_{i+1}]$ | $f'(x_i)$ | $f'(x_{i+1})$ | Trapezoid area |
|------------------------|-----------|----------------|----------------|
| $[1.0,\; 1.5]$        | 2.0       | 3.0            | 1.25           |
| $[1.5,\; 2.0]$        | 3.0       | 4.0            | 1.75           |
| $[2.0,\; 2.5]$        | 4.0       | 5.0            | 2.25           |
| $[2.5,\; 3.0]$        | 5.0       | 6.0            | 2.75           |
| **Total**              |           |                | **8.00** ✓     |

The trapezoid sum converges to $f(3)-f(1) = 8$ as $\Delta x \to 0$, illustrating Part II.

```
f'(x) = 2x
  6 |              *
    |           /
  4 |        *
    |  area=8  (exact)
  2 |  *
    +--+---+---+--
       1   2   3
```

## Proof Sketch

**Part II:**

1. **Partition:** Subdivide $[a, b]$ into subintervals $[x_{i-1}, x_i]$.
2. **Apply MVT on each strip:** By the [[Mean Value Theorem]], for each $i$ there exists $c_i \in (x_{i-1}, x_i)$ with $f(x_i) - f(x_{i-1}) = f'(c_i) \Delta x_i$.
3. **Telescope:** Sum over all strips — the left side telescopes to $f(b) - f(a)$.
4. **Take limit:** As the mesh $\|\Delta\| \to 0$, the right side converges to $\int_a^b f'(x)\,dx$ by definition of the Riemann integral.

## Connections

- [[Mean Value Theorem]] — the key lemma applied subinterval-by-subinterval in the proof
- [[Chain Rule]] — generalises FTC to compositions: $\frac{d}{dx}\int_a^{g(x)} f(t)\,dt = f(g(x)) \cdot g'(x)$
- [[Taylor's Theorem]] — Taylor's theorem can be derived by applying FTC repeatedly ($n+1$ times)
- [[L’Hôpital's Rule]] — evaluation of many limits uses FTC to identify derivative values
- [[Intermediate Value Theorem]] — continuity of $f$ (required by Part I) is also the hypothesis of IVT

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Integral.IntervalIntegral.FundThmCalculus

open MeasureTheory intervalIntegral

/-- Fundamental Theorem of Calculus Part II: the integral of `f'` over `[a, b]`
    equals `f b - f a`. Wraps Mathlib's `integral_eq_sub_of_hasDerivAt`. -/
theorem ftc {f f' : ℝ → ℝ} {a b : ℝ}
    (hderiv : ∀ x ∈ Set.uIcc a b, HasDerivAt f (f' x) x)
    (hint : IntervalIntegrable f' MeasureTheory.volume a b) :
    ∫ x in a..b, f' x = f b - f a :=
  integral_eq_sub_of_hasDerivAt hderiv hint
```
