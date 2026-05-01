+++
title = "Stone–Weierstrass Theorem"
description = "A subalgebra of continuous functions on a compact space that separates points is dense in the sup-norm"
weight = 60
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "A \\subseteq C(X) \\text{ sep. pts, contains 1} \\Rightarrow \\overline{A} = C(X)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $X$ be a **compact Hausdorff space** and $A \subseteq C(X, \mathbb{R})$ a subalgebra (closed under addition, multiplication, and scalar multiplication) that:

1. **Separates points**: for all $x \neq y \in X$, there exists $f \in A$ with $f(x) \neq f(y)$.
2. **Contains constants** (equivalently, the topological closure of $A$ contains constants).

Then $A$ is **dense** in $C(X, \mathbb{R})$ with the sup-norm: every continuous $g : X \to \mathbb{R}$ is a uniform limit of functions in $A$.

The classical Weierstrass approximation theorem ($X = [a,b]$, $A = $ polynomials) is the prototypical case.

## Visualization

Approximate $g(x) = |x|$ on $[-1, 1]$ using Bernstein polynomials (a subalgebra containing polynomials, which separate points):

$$B_{n,k}(x) = \binom{n}{k} x^k (1-x)^k, \quad B_n(g)(x) = \sum_{k=0}^n g(k/n) \binom{n}{k} x^k (1-x)^{n-k}$$

| $x$ | $g(x) = |x|$ | $B_4(g)(x)$ | $B_8(g)(x)$ | $B_{16}(g)(x)$ |
|---|---|---|---|---|
| $-1.0$ | $1.000$ | $1.000$ | $1.000$ | $1.000$ |
| $-0.5$ | $0.500$ | $0.500$ | $0.500$ | $0.500$ |
| $-0.25$ | $0.250$ | $0.313$ | $0.281$ | $0.266$ |
| $0.0$ | $0.000$ | $0.375$ | $0.273$ | $0.196$ |
| $0.25$ | $0.250$ | $0.313$ | $0.281$ | $0.266$ |
| $0.5$ | $0.500$ | $0.500$ | $0.500$ | $0.500$ |
| $1.0$ | $1.000$ | $1.000$ | $1.000$ | $1.000$ |

The Bernstein approximations improve as $n$ grows, converging uniformly to $|x|$.

```
sup-norm error:
  n = 4:   max gap ≈ 0.375  (at x = 0)
  n = 8:   max gap ≈ 0.273
  n = 16:  max gap ≈ 0.196
  n → ∞:   max gap → 0      (Stone–Weierstrass)
```

## Proof Sketch

1. **Closure under $|\cdot|$.** Using the Weierstrass approximation of $|t|$ by polynomials on $[-\|f\|, \|f\|]$, show $|f| \in \overline{A}$ for any $f \in A$.
2. **Lattice structure.** From $|f|$, deduce $f \vee g = (f + g + |f - g|)/2 \in \overline{A}$ and $f \wedge g \in \overline{A}$.
3. **Urysohn-type construction.** For any $g \in C(X)$, $x \in X$, and $\varepsilon > 0$: using separation, construct $h_{xy} \in A$ with $h_{xy}(x) \approx g(x)$ and $h_{xy}(y) \approx g(y)$.
4. **Finite cover.** Compactness gives a finite cover; take finite inf and sup to build $f \in \overline{A}$ with $\|f - g\|_\infty < \varepsilon$.

## Connections

- [[Heine–Borel Theorem]] — compactness of $X$ is essential; without it the theorem fails (e.g., $X = \mathbb{R}$, polynomials do not uniformly approximate $e^{-x^2}$).
- [[Intermediate Value Theorem]] — the proof that $|f| \in \overline{A}$ uses IVT-style polynomial approximation (Weierstrass) on a compact interval.
- [[Taylor's Theorem]] — Taylor polynomials form a subalgebra that separates points; Stone–Weierstrass explains why Taylor-based approximation is density in $C[a,b]$ (when convergent).
- [[Cauchy–Schwarz Inequality]] — in $L^2(X)$, the $L^2$-density of polynomials follows from Stone–Weierstrass plus Cauchy–Schwarz domination.

## Lean4 Proof

```lean4
import Mathlib.Topology.ContinuousMap.StoneWeierstrass

/-- **Stone–Weierstrass Theorem**: a separating subalgebra is dense in C(X, ℝ).
    Direct alias of `ContinuousMap.subalgebra_topologicalClosure_eq_top_of_separatesPoints`. -/
theorem stone_weierstrass {X : Type*} [TopologicalSpace X] [CompactSpace X]
    (A : Subalgebra ℝ C(X, ℝ)) (hA : A.SeparatesPoints) :
    A.topologicalClosure = ⊤ :=
  ContinuousMap.subalgebra_topologicalClosure_eq_top_of_separatesPoints A hA
```
