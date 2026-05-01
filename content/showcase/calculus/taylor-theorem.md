+++
title = "Taylor's Theorem"
description = "Approximate smooth functions by polynomials with explicit error bounds"
weight = 40
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f(x) = \\sum_{k=0}^{n} \\frac{f^{(k)}(x_0)}{k!}(x-x_0)^k + R_n(x)"
prerequisites = ["fundamental-theorem-calculus", "mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

If $f$ is $n$ times differentiable on an open interval containing $x_0$, then for any $x$ in that interval:

$$f(x) = \sum_{k=0}^{n} \frac{f^{(k)}(x_0)}{k!}(x - x_0)^k + R_n(x)$$

**Lagrange remainder:** There exists $c$ between $x_0$ and $x$ such that:

$$R_n(x) = \frac{f^{(n+1)}(c)}{(n+1)!}(x - x_0)^{n+1}$$

The theorem gives a polynomial approximation to $f$ near $x_0$ with a rigorous bound on the error.

## Visualization

Approximating $\sin(x)$ near $x_0 = 0$ using Taylor polynomials of increasing degree:

| $n$ | Taylor polynomial $T_n(x)$                              | $T_n(0.5)$ | error vs $\sin(0.5) \approx 0.4794$ |
|-----|--------------------------------------------------------|-----------|--------------------------------------|
| 1   | $x$                                                    | 0.5000    | 0.0206                               |
| 3   | $x - \dfrac{x^3}{6}$                                  | 0.4792    | 0.0002                               |
| 5   | $x - \dfrac{x^3}{6} + \dfrac{x^5}{120}$              | 0.47943   | $< 10^{-5}$                          |
| 7   | $x - \dfrac{x^3}{6} + \dfrac{x^5}{120} - \dfrac{x^7}{5040}$ | 0.479426  | $< 10^{-7}$          |

The Lagrange remainder for $T_n$ at $x = 0.5$: $|R_n(0.5)| \le \dfrac{(0.5)^{n+1}}{(n+1)!}$, shrinking rapidly as $n$ grows.

```
    sin(x)  — exact
    T₁(x)   ……… degree 1
    T₃(x)   - - degree 3
    T₅(x)   --- degree 5

 1 |   .--sin
   |  /  T₅ ≈ sin here
   | /
 0 +-------> x
   |
-1 |
```

## Proof Sketch

1. **Base case ($n = 0$):** Directly apply the [[Mean Value Theorem]] — $f(x) - f(x_0) = f'(c)(x - x_0)$.
2. **Induction:** Assume $T_{n-1}(x)$ approximates $f$ with remainder $R_{n-1}$.
3. **Apply the [[Fundamental Theorem of Calculus]]:** Write $R_{n-1}(x) = \int_{x_0}^{x} \frac{(x-t)^{n-1}}{(n-1)!} f^{(n)}(t)\,dt$.
4. **Lagrange form:** Apply the MVT to the integral representation of $R_n$ using the weight function $(x-t)^n$. A $c$ exists with the Lagrange form of the remainder.
5. **Conclude:** The remainder $|R_n(x)| \le \frac{M}{(n+1)!}|x - x_0|^{n+1}$ where $M = \sup |f^{(n+1)}|$.

## Connections

- [[Mean Value Theorem]] — the $n=0$ base case and the Lagrange remainder step both use MVT
- [[Fundamental Theorem of Calculus]] — the integral remainder representation of $R_n$ comes from FTC applied $n$ times
- [[Chain Rule]] — used when computing $f^{(k)}(x_0)$ for composite functions
- [[L’Hôpital's Rule]] — Taylor expansions provide an alternative route to L'Hôpital-type limits
- [[Intermediate Value Theorem]] — continuity required for $f^{(n)}$ invokes IVT-like arguments

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.Taylor

open Set

/-- Taylor's theorem with the Lagrange remainder: there exists `c ∈ (x₀, x)` such that
    the error between `f(x)` and the degree-`n` Taylor polynomial at `x₀` is
    `f^(n+1)(c) · (x - x₀)^(n+1) / (n+1)!`.
    Wraps Mathlib's `taylor_mean_remainder_lagrange`. -/
theorem taylor_lagrange_remainder {f : ℝ → ℝ} {x x₀ : ℝ} {n : ℕ}
    (hx : x₀ < x)
    (hf : ContDiffOn ℝ n f (Icc x₀ x))
    (hf' : DifferentiableOn ℝ (iteratedDerivWithin n f (Icc x₀ x)) (Ioo x₀ x)) :
    ∃ c ∈ Ioo x₀ x,
      f x - taylorWithinEval f n (Icc x₀ x) x₀ x =
        iteratedDerivWithin (n + 1) f (Icc x₀ x) c * (x - x₀) ^ (n + 1) / (n + 1)! :=
  taylor_mean_remainder_lagrange hx hf hf'
```
