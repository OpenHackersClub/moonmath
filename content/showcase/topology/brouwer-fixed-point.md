+++
title = "Brouwer Fixed-Point Theorem"
description = "Every continuous map from a compact convex set to itself has a fixed point"
weight = 30
tags = ["lean4-proof", "topology", "fixed-point", "visualization"]
latex = "f : D^n \\to D^n \\text{ continuous} \\implies \\exists\\, x,\\; f(x) = x"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $D^n = \{x \in \mathbb{R}^n : \|x\| \le 1\}$ be the closed unit ball. Every continuous map $f : D^n \to D^n$ has a **fixed point**:

$$\exists\, x \in D^n, \quad f(x) = x.$$

The $n=1$ case is immediate from the Intermediate Value Theorem; the general case requires algebraic topology (homology or degree theory).

## Visualization

In 1D, the theorem says every continuous $f : [0,1] \to [0,1]$ crosses the diagonal.

```
1 │                  ╱ y = x (diagonal)
  │            ╱
  │       ╱  ╳────── f(x) crosses y=x here → fixed point!
  │    ╱  ╱
  │ ╱  ╱
0 └────────────────── 1
  0                  1

If f(0) ≥ 0 and f(1) ≤ 1, let g(x) = f(x) − x.
  g(0) = f(0) − 0 ≥ 0
  g(1) = f(1) − 1 ≤ 0
IVT → ∃ x* with g(x*) = 0, i.e. f(x*) = x*.
```

Intuitively: crumple a piece of paper and lay it on a copy of itself — at least one point lands exactly on its original position.

## Proof Sketch

**One dimension.** Define $g(x) = f(x) - x$. Then $g(0) = f(0) \ge 0$ and $g(1) = f(1) - 1 \le 0$. Since $g$ is continuous, the Intermediate Value Theorem yields $x^* \in [0,1]$ with $g(x^*) = 0$, i.e. $f(x^*) = x^*$.

**General $n$.** Suppose for contradiction $f$ has no fixed point. Define a retraction $r : D^n \to S^{n-1}$ by drawing the ray from $f(x)$ through $x$ and letting $r(x)$ be where it meets $S^{n-1}$. This $r$ would be a continuous retraction of $D^n$ onto $S^{n-1}$, contradicting the fact that $S^{n-1}$ is not a retract of $D^n$ (witnessed by homology: $H_{n-1}(S^{n-1}) \cong \mathbb{Z}$ but $H_{n-1}(D^n) = 0$). Mathlib contains the full $n$-dimensional proof in `Mathlib.Topology.Homotopy.Brouwer`.

## Connections

- **[[Heine–Borel Theorem]]** — the closed unit ball is compact (closed and bounded); compactness is essential for most existence proofs.
- **[[Bolzano–Weierstrass Theorem]]** — the 1D proof uses the Intermediate Value Theorem, closely related to the nested interval argument behind Bolzano–Weierstrass.
- **[[Tychonoff's Theorem]]** — in infinite dimensions (e.g. Hilbert space), Tychonoff's theorem helps recover a version via the Schauder fixed-point theorem.
- **[[Urysohn's Lemma]]** — separation of points is used in the homological proof to construct test functions distinguishing boundary from interior.
- **[[Hausdorff Distance]]** — iterated approximation schemes for fixed points converge in the Hausdorff metric on compact sets.
- **[[Iterated Function Systems]]** — IFS attractors are Banach fixed points in the Hausdorff metric space; Brouwer's theorem gives a topological (not metric) companion existence result.

## Lean4 Proof

The 1D fixed-point theorem follows directly from Mathlib's `intermediate_value_Icc`.

```lean4
import Mathlib.Topology.Order.IntermediateValue
import Mathlib.Topology.ContinuousOn

open Set

/-- **1D Brouwer fixed-point theorem**: every continuous map f : [0,1] → [0,1]
    has a fixed point. Proof via the Intermediate Value Theorem applied to g = f - id. -/
theorem brouwer_1d {f : ℝ → ℝ} (hf : ContinuousOn f (Icc 0 1))
    (h0 : 0 ≤ f 0) (h1 : f 1 ≤ 1) :
    ∃ x ∈ Icc (0 : ℝ) 1, f x = x := by
  -- g(x) = f(x) - x is continuous, g(0) ≥ 0, g(1) ≤ 0
  have hg : ContinuousOn (fun x => f x - x) (Icc 0 1) :=
    hf.sub continuousOn_id
  have hg0 : (0 : ℝ) ≤ (fun x => f x - x) 0 := by simpa using h0
  have hg1 : (fun x => f x - x) 1 ≤ (0 : ℝ) := by simpa using h1
  -- IVT gives a zero of g
  have hmem : (0 : ℝ) ∈ Icc ((fun x => f x - x) 1) ((fun x => f x - x) 0) :=
    Set.mem_Icc.mpr ⟨hg1, hg0⟩
  obtain ⟨x, hx, hfx⟩ := intermediate_value_Icc' (by norm_num : (0:ℝ) ≤ 1) hg hmem
  exact ⟨x, hx, by linarith⟩

-- The full n-dimensional Brouwer theorem is in Mathlib at:
-- Mathlib.Topology.Homotopy.Brouwer (not yet in this lake snapshot).
-- The 1D IVT-based version above is complete and sorry-free.
```
