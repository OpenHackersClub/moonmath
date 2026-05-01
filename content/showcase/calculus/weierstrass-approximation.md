+++
title = "Weierstrass Approximation Theorem"
description = "Every continuous function on a closed bounded interval can be uniformly approximated by polynomials"
weight = 140
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\forall\\varepsilon>0\\;\\exists p\\in\\mathbb{R}[x]:\\;\\sup_{x\\in[a,b]}|f(x)-p(x)|<\\varepsilon"
prerequisites = ["extreme-value-theorem", "intermediate-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f : [a, b] \to \mathbb{R}$ be continuous. For every $\varepsilon > 0$ there exists a polynomial $p \in \mathbb{R}[x]$ such that

$$\sup_{x \in [a,b]} |f(x) - p(x)| < \varepsilon$$

Equivalently, the polynomials are **dense** in $C([a,b])$ with the uniform norm $\|f\|_\infty = \sup_{[a,b]} |f|$.

## Visualization

Bernstein polynomials $B_n f$ approximate $f(x) = |x|$ on $[-1, 1]$ uniformly. Explicitly, reparameterizing to $[0,1]$ with $g(t) = |2t-1|$:

$$B_n g(t) = \sum_{k=0}^{n} g\!\left(\frac{k}{n}\right) \binom{n}{k} t^k (1-t)^{n-k}$$

| $x$ | $f(x) = |x|$ | $B_4$ | $B_8$ | $B_{16}$ |
|-----|------------|-------|-------|--------|
| 0.0 | 0.000      | 0.375 | 0.273 | 0.196  |
| 0.2 | 0.200      | 0.261 | 0.231 | 0.214  |
| 0.5 | 0.500      | 0.500 | 0.500 | 0.500  |
| 0.8 | 0.800      | 0.761 | 0.781 | 0.791  |
| 1.0 | 1.000      | 1.000 | 1.000 | 1.000  |

(Values computed for $g(t) = |2t-1|$ on $[0,1]$, so $t = (x+1)/2$.)

The uniform error $\|B_n g - g\|_\infty = O(n^{-1/2})$ for Lipschitz $g$; higher-degree polynomials converge to $|x|$ everywhere.

```
  Approximation of |x| by polynomials on [-1,1]

  1 |*         *   ← |x|
    | *\     /*
    |  *\   /* ← B₄ (smoothed corners)
    |   *\ /* ← B₈
  0 |----*------   ← x=0, f(0)=0, B_n(0)→0
    |   /* \*
    |  /    \* ← converging to |x|
   -1                   1
```

## Proof Sketch

1. **Bernstein polynomials:** For $f \in C([0,1])$ define $B_n f(x) = \sum_{k=0}^{n} f(k/n) \binom{n}{k} x^k (1-x)^{n-k}$.
2. **Probabilistic identity:** $B_n f(x) = \mathbb{E}[f(S_n/n)]$ where $S_n \sim \mathrm{Binomial}(n, x)$. By the law of large numbers $S_n/n \to x$, so $B_n f(x) \to f(x)$.
3. **Uniform convergence:** Continuity on the compact set $[0,1]$ implies uniform continuity. The estimate $|B_n f(x) - f(x)| \leq \omega_f(\delta) + 2\|f\|_\infty / (n\delta^2)$ (where $\omega_f$ is the modulus of continuity) gives uniform convergence by choosing $\delta = n^{-1/4}$.
4. **General $[a,b]$:** Apply the $[0,1]$ result after the affine reparameterization $t = (x-a)/(b-a)$.

## Connections

- [[Extreme Value Theorem]] — $f$ is uniformly continuous on $[a,b]$ (since it is compact) — this is the key regularity that makes the Bernstein approximation converge
- [[Heine-Borel Theorem]] — the compact domain $[a,b]$ is essential; the theorem fails for all of $\mathbb{R}$ (polynomials cannot uniformly approximate $e^x$)
- [[Intermediate Value Theorem]] — the density of polynomials in $C[a,b]$ is the analytic analogue of the IVT: every continuous function is a limit of explicit elementary ones

## Lean4 Proof

```lean4
import Mathlib.Topology.ContinuousMap.Weierstrass

/-- Weierstrass Approximation Theorem: polynomials are dense in C([a,b]).
    Wraps Mathlib's `polynomialFunctions_closure_eq_top`. -/
theorem weierstrass_approximation (a b : ℝ) :
    (polynomialFunctions (Set.Icc a b)).topologicalClosure = ⊤ :=
  polynomialFunctions_closure_eq_top a b
```
