+++
title = "Monotone Convergence Theorem"
description = "A bounded monotone sequence of reals always converges — the supremum is the limit"
weight = 20
tags = ["lean4-proof", "analysis", "visualization"]
latex = "f \\text{ monotone},\\; \\sup_n f(n) < \\infty \\implies f(n) \\to \\sup_n f(n)"
prerequisites = []
lean4_status = "complete"
+++

The **Monotone Convergence Theorem** (for sequences) is one of the fundamental completeness properties of the real line: a sequence that is non-decreasing and bounded above cannot escape to infinity, so it must settle somewhere. That somewhere is exactly its supremum.

## Statement

Let $f : \mathbb{N} \to \mathbb{R}$ be monotone non-decreasing, i.e.\ $f(m) \leq f(n)$ whenever $m \leq n$. If the set $\{f(n) : n \in \mathbb{N}\}$ is bounded above, then $f$ converges and

$$\lim_{n \to \infty} f(n) = \sup_{n \in \mathbb{N}} f(n).$$

The dual statement holds for antitone sequences bounded below (they converge to their infimum).

## Visualization

Take $f(n) = 1 - 1/n$ (for $n \geq 1$), which is monotone increasing and bounded above by $1$.

```
  n  |  f(n) = 1 - 1/n  |  sup so far
-----|-------------------|------------
  1  |  0.000            |  0.000
  2  |  0.500            |  0.500
  4  |  0.750            |  0.750
  8  |  0.875            |  0.875
 16  |  0.937            |  0.937
 32  |  0.969            |  0.969
 64  |  0.984            |  0.984
128  |  0.992            |  0.992
```

The supremum is $1$, approached but never reached from below — a staircase climbing toward a ceiling.

```
1.0  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (sup = limit)
     ↑  ↑    ↑        ↑
 f(1) f(2)  f(4)     f(8)  ···
```

## Proof Sketch

Let $L = \sup_n f(n)$. By definition of supremum, $f(n) \leq L$ for all $n$. Fix $\varepsilon > 0$: since $L - \varepsilon$ is not an upper bound, there exists $N$ with $f(N) > L - \varepsilon$. By monotonicity, for $n \geq N$:

$$L - \varepsilon < f(N) \leq f(n) \leq L,$$

so $|f(n) - L| < \varepsilon$.

This proof uses only that $\mathbb{R}$ has the least-upper-bound property — it fails in $\mathbb{Q}$, where bounded monotone sequences can converge to irrational limits outside the space.

## Connections

The theorem is the bridge between order completeness and metric completeness. It implies that every [[Cauchy Criterion|Cauchy sequence]] in $\mathbb{R}$ converges (via the Bolzano-Weierstrass subsequence argument). It is also the backbone of the Lebesgue Monotone Convergence Theorem (for non-negative measurable functions), which appears in integration theory. Within this section, the [[Squeeze Theorem]] handles convergence by bounding a sequence above and below, while the [[Cauchy Criterion]] characterises convergence without knowing the limit in advance.

## Lean4 Proof

```lean4
import Mathlib.Topology.Order.MonotoneConvergence

open Filter Topology

/-- A monotone, bounded-above sequence of reals converges to its supremum. -/
theorem monotone_convergence {f : ℕ → ℝ} (hf : Monotone f)
    (hb : BddAbove (Set.range f)) :
    Tendsto f atTop (𝓝 (⨆ n, f n)) :=
  tendsto_atTop_ciSup hf hb
```
