+++
title = "Cauchy Criterion"
description = "A sequence converges if and only if its terms eventually become arbitrarily close to each other — no candidate limit required"
weight = 30
tags = ["lean4-proof", "analysis", "visualization"]
latex = "\\forall\\varepsilon>0\\;\\exists N:\\; m,n \\geq N \\implies |a_m - a_n| < \\varepsilon"
prerequisites = []
lean4_status = "complete"
+++

The **Cauchy criterion** reframes convergence: instead of asking whether a sequence approaches some target value, it asks whether the sequence's terms eventually cluster among themselves. This is powerful because we can verify the criterion without knowing the limit — and in complete spaces, that is enough to guarantee one exists.

## Statement

A sequence $(a_n)$ in a metric space $(X, d)$ is a **Cauchy sequence** if

$$\forall \varepsilon > 0,\; \exists N \in \mathbb{N} : m, n \geq N \implies d(a_m, a_n) < \varepsilon.$$

**Theorem.** Every Cauchy sequence in a **complete** metric space converges. Conversely, every convergent sequence is Cauchy.

A metric space in which every Cauchy sequence converges is called **complete**. The real line $\mathbb{R}$ is complete; the rationals $\mathbb{Q}$ are not.

## Visualization

The partial sums $s_n = \sum_{k=1}^n \frac{1}{k^2}$ form a Cauchy sequence in $\mathbb{Q}$ that converges to $\pi^2/6 \notin \mathbb{Q}$:

```
  n  |  s_n (partial sum of 1/k²)  |  |s_n - s_{n-1}| = 1/n²
-----|-----------------------------|-----------------------
  1  |  1.000 000                  |  —
  2  |  1.250 000                  |  0.250 000
  5  |  1.463 611                  |  0.040 000
 10  |  1.549 768                  |  0.010 000
 20  |  1.596 163                  |  0.002 500
 50  |  1.625 133                  |  0.000 400
100  |  1.634 984                  |  0.000 100
```

The increments $1/n^2$ go to zero, so for $m, n \geq N$:

$$|s_m - s_n| \leq \sum_{k=N}^{\infty} \frac{1}{k^2} \approx \frac{1}{N} \to 0.$$

In $\mathbb{Q}$ this sequence is Cauchy but has no rational limit — the series sums to $\pi^2/6$, an irrational. In $\mathbb{R}$ (which is complete) it does converge.

A contrasting non-Cauchy sequence: $a_n = (-1)^n$ oscillates between $-1$ and $1$, so $|a_{n+1} - a_n| = 2$ for every $n$ — never small.

## Proof Sketch

**(Cauchy $\Rightarrow$ convergent in $\mathbb{R}$.)** A Cauchy sequence is bounded (apply the criterion with $\varepsilon = 1$ to get a bound beyond some $N$, then take the max with the first $N$ terms). By Bolzano-Weierstrass it has a convergent subsequence $a_{n_k} \to L$. The full sequence then converges to $L$: given $\varepsilon > 0$, pick $N$ so that $d(a_m, a_n) < \varepsilon/2$ for $m, n \geq N$, and pick $k$ with $n_k \geq N$ and $d(a_{n_k}, L) < \varepsilon/2$; then $d(a_n, L) < \varepsilon$.

**(Convergent $\Rightarrow$ Cauchy.)** If $a_n \to L$, pick $N$ with $d(a_n, L) < \varepsilon/2$ for $n \geq N$; then $d(a_m, a_n) \leq d(a_m, L) + d(L, a_n) < \varepsilon$.

## Connections

The Cauchy criterion is the internal completeness statement that [[Monotone Convergence Theorem|Monotone Convergence]] expresses via order. In normed spaces it leads to the notion of **Banach spaces**, where the [[Iterated Function Systems]] contraction argument (and hence [[Hausdorff Distance]] completeness) is carried out. The criterion also characterises uniform convergence of function series, which underlies the [[Liouville's Theorem|Liouville theorem]] proof via the Cauchy integral formula.

## Lean4 Proof

```lean4
import Mathlib.Topology.EMetricSpace.Basic
import Mathlib.Topology.MetricSpace.Basic

open Filter Topology

/-- Every Cauchy sequence in a complete metric space converges. -/
theorem cauchy_complete {X : Type*} [MetricSpace X] [CompleteSpace X]
    {x : ℕ → X} (hx : CauchySeq x) :
    ∃ a, Tendsto x atTop (𝓝 a) :=
  cauchySeq_tendsto_of_complete hx
```
