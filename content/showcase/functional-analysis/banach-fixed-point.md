+++
title = "Banach Fixed Point Theorem"
description = "Every contraction on a complete metric space has a unique fixed point, reached by iterating the map"
weight = 50
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "T : X \\to X\\text{ contraction} \\Rightarrow \\exists!\\, x^* = T(x^*)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $(X, d)$ be a **complete metric space** and $T : X \to X$ a **contraction mapping**: there exists $K \in [0, 1)$ such that

$$d(Tx, Ty) \le K \cdot d(x, y) \quad \text{for all } x, y \in X.$$

Then $T$ has a **unique fixed point** $x^* \in X$ with $T(x^*) = x^*$. Moreover, for any starting point $x_0 \in X$, the iterates $T^n(x_0)$ converge to $x^*$ with error bound

$$d(T^n(x_0),\, x^*) \le \frac{K^n}{1 - K}\, d(x_0, T(x_0)).$$

## Visualization

Take $T(x) = x/2 + 1$ on $\mathbb{R}$ with $K = 1/2$. Fixed point: $x^* = x^*/2 + 1 \Rightarrow x^* = 2$.

| $n$ | $T^n(x_0)$ with $x_0 = 0$ | $|T^n(x_0) - 2|$ | bound $K^n d(x_0,Tx_0)/(1-K)$ |
|---|---|---|---|
| 0 | $0.000$ | $2.000$ | $2.000$ |
| 1 | $1.000$ | $1.000$ | $1.000$ |
| 2 | $1.500$ | $0.500$ | $0.500$ |
| 3 | $1.750$ | $0.250$ | $0.250$ |
| 4 | $1.875$ | $0.125$ | $0.125$ |
| 5 | $1.938$ | $0.063$ | $0.063$ |
| 10 | $1.999$ | $0.001$ | $0.001$ |

```
x-axis: 0     1     1.5   1.75  ...  2
          ──▶───▶────▶──────▶──────▶ x* = 2

Each arrow is one application of T(x) = x/2 + 1.
Distance to x* halves each step (K = 1/2).
```

## Proof Sketch

1. **Cauchy sequence.** Show $\{T^n(x_0)\}$ is Cauchy: $d(T^m x_0, T^n x_0) \le \frac{K^{\min(m,n)}}{1-K} d(x_0, Tx_0) \to 0$.
2. **Convergence.** By completeness, $T^n(x_0) \to x^*$ for some $x^* \in X$.
3. **Fixed point.** Pass to the limit in $T(T^n x_0) = T^{n+1} x_0$; by continuity of $T$, $T(x^*) = x^*$.
4. **Uniqueness.** If $T(y) = y$ also, then $d(x^*, y) = d(Tx^*, Ty) \le K d(x^*, y)$, so $(1-K) d(x^*, y) \le 0$, hence $y = x^*$.

## Connections

- [[Brouwer Fixed-Point Theorem]] — the Brouwer theorem (topological fixed point) covers compact convex sets in $\mathbb{R}^n$ without assuming contraction; Banach's theorem covers complete metric spaces without assuming compactness.
- [[Cauchy Criterion]] — the Cauchy sequence construction in the proof uses exactly the Cauchy criterion for convergence in complete spaces.
- [[Iterated Function Systems]] — IFS attractors are fixed points of the Hutchinson operator (a contraction on the Hausdorff-metric space of compact sets), making Banach's theorem the engine behind fractal geometry.
- [[Hausdorff Distance]] — the space of non-empty compact subsets of a complete metric space is complete under the Hausdorff metric, so the Banach theorem applies to prove IFS attractor existence.

## Lean4 Proof

```lean4
import Mathlib.Topology.MetricSpace.Contracting

/-- **Banach Fixed Point Theorem**: contraction on complete metric space has unique fixed point.
    Uses `ContractingWith.fixedPoint_isFixedPt` and `ContractingWith.fixedPoint_unique`. -/
theorem banach_fixed_point
    {X : Type*} [MetricSpace X] [CompleteSpace X] [Nonempty X]
    {K : NNReal} {f : X → X} (hf : ContractingWith K f) :
    ∃! x : X, f x = x :=
  ⟨ContractingWith.fixedPoint f hf,
   ContractingWith.fixedPoint_isFixedPt hf,
   fun y hy => ContractingWith.fixedPoint_unique hf hy⟩
```
