+++
title = "Bolzano–Weierstrass Theorem"
description = "Every bounded sequence in ℝⁿ has a convergent subsequence"
weight = 20
tags = ["lean4-proof", "topology", "sequences", "compactness"]
latex = "\\text{bounded } (x_n) \\subseteq \\mathbb{R}^n \\implies \\exists\\, \\text{convergent subsequence}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $(x_n)_{n \in \mathbb{N}}$ be a sequence in $\mathbb{R}^n$ whose range is bounded — that is, $\{x_n : n \in \mathbb{N}\} \subseteq B_R(0)$ for some $R < \infty$. Then there exists a strictly increasing $\varphi : \mathbb{N} \to \mathbb{N}$ and a point $a \in \mathbb{R}^n$ such that

$$x_{\varphi(0)}, x_{\varphi(1)}, x_{\varphi(2)}, \ldots \to a.$$

## Visualization

A bounded sequence in $[0,1]$ need not converge, but some subsequence always does.

```
Sequence in [0,1] — values plotted vertically:

n:   0    1    2    3    4    5    6    7    8    9   10 ...
x_n: 0.8  0.2  0.9  0.1  0.85 0.15 0.91 0.09 0.88 0.11 0.9

Subsequence ↓ (even indices, selected by φ(k)=2k):
     0.8       0.9       0.85      0.91      0.88      0.9 → converges near 0.9

Subsequence ↓ (odd indices, selected by φ(k)=2k+1):
          0.2       0.1       0.15      0.09      0.11 → converges near 0.1

Both subsequences converge, to different limits.
The original sequence oscillates between ~0.1 and ~0.9 but does NOT converge.
```

Bolzano–Weierstrass guarantees the existence of at least one such subsequence — it does not say the whole sequence converges, only that you can always find an infinite monotone index selection that does.

## Proof Sketch

By [[Heine–Borel Theorem]], a closed ball $\overline{B_R(0)} \subseteq \mathbb{R}^n$ is compact. A compact metric space is *sequentially compact* — every sequence has a convergent subsequence. Since $(x_n)$ is eventually in $\overline{B_R(0)}$ (up to passing to a tail), sequential compactness yields the subsequence.

Alternatively, for $n=1$: nest closed intervals by bisection — at each step, one half-interval contains infinitely many terms; pick that half and repeat. The intersection of the nested intervals is a single point $a$, and the construction yields an explicit convergent subsequence.

## Connections

- **[[Heine–Borel Theorem]]** — the ambient compact box supplies sequential compactness.
- **[[Brouwer Fixed-Point Theorem]]** — sequential compactness arguments underpin degree theory.
- **[[Tychonoff Theorem]]** — in infinite dimensions, compactness (and hence Bolzano–Weierstrass) fails for the norm topology; Tychonoff's theorem recovers it in the product (weak) topology.
- **[[Hausdorff Distance]]** — the Blaschke selection theorem (Bolzano–Weierstrass for compact sets) follows: a bounded sequence of compact sets has a subsequence convergent in $d_H$.
- **[[Urysohn Lemma]]** — Urysohn functions are used to separate cluster points; the separation relies on the same normal-space axioms underlying convergence arguments.
- **[[Iterated Function Systems]]** — the attractor is the unique fixed compact set; Bolzano–Weierstrass guarantees that any sequence of approximating sets has cluster points.

## Lean4 Proof

```lean4
import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Bounded

open Filter Bornology Metric Topology

/-- **Bolzano–Weierstrass**: every sequence taking values in a bounded set in a
    proper metric space (e.g. ℝ or ℝⁿ) has a convergent subsequence.
    Mathlib: `tendsto_subseq_of_bounded` in `Mathlib.Topology.MetricSpace.Sequences`. -/
theorem bolzano_weierstrass {X : Type*} [MetricSpace X] [ProperSpace X]
    {s : Set X} (hs : IsBounded s) {x : ℕ → X} (hx : ∀ n, x n ∈ s) :
    ∃ a ∈ closure s, ∃ φ : ℕ → ℕ, StrictMono φ ∧ Tendsto (x ∘ φ) atTop (𝓝 a) :=
  tendsto_subseq_of_bounded hs hx

/-- Specialised to ℝ: a bounded real sequence has a convergent subsequence. -/
theorem bolzano_weierstrass_real {x : ℕ → ℝ} (R : ℝ) (hR : ∀ n, |x n| ≤ R) :
    ∃ (φ : ℕ → ℕ) (a : ℝ), StrictMono φ ∧ Tendsto (x ∘ φ) atTop (𝓝 a) := by
  have hs : IsBounded (Set.Icc (-R) R) := isBounded_Icc (-R) R
  have hx : ∀ n, x n ∈ Set.Icc (-R) R := fun n => Set.mem_Icc.mpr ⟨by linarith [hR n], hR n⟩
  obtain ⟨a, _, φ, hφ, hconv⟩ := tendsto_subseq_of_bounded hs hx
  exact ⟨φ, a, hφ, hconv⟩
```
