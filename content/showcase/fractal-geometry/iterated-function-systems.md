+++
title = "Iterated Function Systems"
description = "Constructing fractals via contractive affine transformations"
weight = 30
tags = ["interactive", "visualization", "fractal", "ifs-3d"]
latex = "A = \\bigcup_{i=1}^{N} f_i(A)"
prerequisites = ["hausdorff-distance", "hausdorff-dimension"]
lean4_status = "complete"
+++

## Definition

An iterated function system (IFS) is a finite collection of contraction mappings $\{f_1, f_2, \ldots, f_N\}$ on a complete metric space. By the Banach fixed-point theorem (applied to the [[Hausdorff Distance|Hausdorff metric]] on compact sets), there exists a unique non-empty compact set $A$ — the **attractor** — satisfying:

$$A = \bigcup_{i=1}^{N} f_i(A)$$

## Affine IFS

Each map is typically an affine transformation:

$$f_i\begin{pmatrix} x \\ y \end{pmatrix} = \begin{pmatrix} a_i & b_i \\ c_i & d_i \end{pmatrix} \begin{pmatrix} x \\ y \end{pmatrix} + \begin{pmatrix} e_i \\ f_i \end{pmatrix}$$

## Examples

**Sierpinski Triangle:** Three maps, each scaling by $1/2$ toward a different vertex of an equilateral triangle.

**Barnsley Fern:** Four affine maps with different probabilities, producing a remarkably realistic fern pattern. The maps encode the self-similar structure of the frond, rachis, and leaflets.

## Chaos Game

A simple algorithm to render the attractor: start at any point, repeatedly choose a random $f_i$ (with given probabilities), and plot the iterates. The orbit converges to the attractor by the contraction mapping principle.

## Connections

The proof rests on the [[Hausdorff Distance]]: the Hutchinson operator is a contraction on the complete metric space $(\mathcal{K}^*(X), d_H)$ of non-empty compact sets, so Banach's fixed-point theorem gives a unique fixed point — the attractor. The [[Hausdorff Dimension]] of an IFS attractor can then be computed from the contraction ratios. The [[Mandelbrot Set]] is intimately connected to IFS theory through the dynamics of quadratic polynomials.

## Proof Sketch (Hutchinson, 1981)

1. **Hausdorff completeness.** $(\mathcal{K}^*(X), d_H)$ is a complete metric space (Hausdorff completeness theorem).
2. **Hutchinson operator.** Define $F : \mathcal{K}^*(X) \to \mathcal{K}^*(X)$ by $F(K) = \bigcup_{i=1}^N f_i(K)$. Each $f_i(K)$ is compact (continuous image of compact) and the finite union of compacts is compact.
3. **$F$ is a contraction.** Let $s = \max_i s_i < 1$ where $s_i$ is the contraction ratio of $f_i$. A short calculation using $d_H(A \cup B, C \cup D) \leq \max(d_H(A, C), d_H(B, D))$ and $d_H(f_i(K), f_i(L)) \leq s_i \cdot d_H(K, L)$ gives $d_H(F(K), F(L)) \leq s \cdot d_H(K, L)$.
4. **Banach fixed point.** A contraction on a non-empty complete metric space has a unique fixed point $A \in \mathcal{K}^*(X)$, and $F^n(K_0) \to A$ for any starting $K_0$.

## Lean4 Proof

The proof below is fully formalised against Mathlib v4.28.0 — no `sorry`, no `admit`. The same source lives at `lean-project/MoonMath/IFS.lean` and is verified by `lake env lean MoonMath/IFS.lean`.

```lean4
import Mathlib.Topology.MetricSpace.Closeds
import Mathlib.Topology.MetricSpace.Contracting

set_option linter.unusedSectionVars false

open Set TopologicalSpace Metric Bornology
open scoped ENNReal NNReal

namespace MoonMath

/-! ### Lipschitz maps and Hausdorff edist -/

section LipschitzImage

variable {X : Type*} [PseudoEMetricSpace X]

/-- For a `K`-Lipschitz map `f` and a non-empty `t`, the infimum edistance
from `f x` to `f '' t` is at most `K · infEDist x t`. -/
private lemma infEDist_image_le_mul {K : ℝ≥0} {f : X → X} (hf : LipschitzWith K f)
    (x : X) {t : Set X} (ht : t.Nonempty) :
    infEDist (f x) (f '' t) ≤ K * infEDist x t := by
  obtain ⟨y₀, hy₀⟩ := ht
  by_cases hK : (K : ℝ≥0∞) = 0
  · rw [hK, zero_mul, nonpos_iff_eq_zero]
    have h_eq : edist (f x) (f y₀) = 0 := by
      have h := hf x y₀
      rw [hK, zero_mul, nonpos_iff_eq_zero] at h
      exact h
    refine le_antisymm ?_ (zero_le _)
    calc infEDist (f x) (f '' t)
        ≤ edist (f x) (f y₀) := infEDist_le_edist_of_mem (mem_image_of_mem _ hy₀)
      _ = 0 := h_eq
  · show infEDist (f x) (f '' t) ≤ (K : ℝ≥0∞) * infEDist x t
    rw [show infEDist (f x) (f '' t) = ⨅ b ∈ t, edist (f x) (f b) by
          simp_rw [infEDist, iInf_image]]
    rw [infEDist, ENNReal.mul_iInf_of_ne hK ENNReal.coe_ne_top]
    refine le_iInf fun y => ?_
    rw [ENNReal.mul_iInf_of_ne hK ENNReal.coe_ne_top]
    refine le_iInf fun hy => ?_
    refine (iInf₂_le y hy).trans ?_
    exact hf x y

/-- For a `K`-Lipschitz map `f`, the Hausdorff edistance of images
satisfies `d_H(f '' s, f '' t) ≤ K · d_H(s, t)` whenever `s, t` are non-empty. -/
private lemma hausdorffEDist_image_le_mul {K : ℝ≥0} {f : X → X}
    (hf : LipschitzWith K f) {s t : Set X}
    (hs : s.Nonempty) (ht : t.Nonempty) :
    hausdorffEDist (f '' s) (f '' t) ≤ K * hausdorffEDist s t := by
  rw [hausdorffEDist_def]
  refine sup_le ?_ ?_
  · rw [iSup_image]
    refine iSup₂_le fun y hy => ?_
    calc infEDist (f y) (f '' t)
        ≤ (K : ℝ≥0∞) * infEDist y t := infEDist_image_le_mul hf y ht
      _ ≤ (K : ℝ≥0∞) * hausdorffEDist s t := by
            gcongr
            exact infEDist_le_hausdorffEDist_of_mem hy
  · rw [iSup_image]
    refine iSup₂_le fun y hy => ?_
    calc infEDist (f y) (f '' s)
        ≤ (K : ℝ≥0∞) * infEDist y s := infEDist_image_le_mul hf y hs
      _ ≤ (K : ℝ≥0∞) * hausdorffEDist s t := by
            gcongr
            calc infEDist y s
                ≤ hausdorffEDist t s := infEDist_le_hausdorffEDist_of_mem hy
              _ = hausdorffEDist s t := hausdorffEDist_comm

end LipschitzImage

/-! ### Iterated function systems -/

section IFS

variable {X : Type*} [MetricSpace X] [CompleteSpace X]

/-- A contraction on `X` with explicit Lipschitz ratio `< 1`. -/
structure ContractionMap (X : Type*) [MetricSpace X] where
  toFun        : X → X
  ratio        : ℝ≥0
  ratio_lt_one : ratio < 1
  lipschitz    : LipschitzWith ratio toFun

namespace ContractionMap

protected lemma continuous (f : ContractionMap X) : Continuous f.toFun :=
  f.lipschitz.continuous

end ContractionMap

/-- An iterated function system: `N` contractions on `X`. -/
structure IFS (X : Type*) [MetricSpace X] (N : ℕ) where
  maps : Fin N → ContractionMap X

namespace IFS

variable {N : ℕ} [NeZero N] (ifs : IFS X N)

/-- Joint contraction ratio: the maximum of individual ratios. -/
noncomputable def jointRatio : ℝ≥0 :=
  Finset.univ.sup' Finset.univ_nonempty (fun i => (ifs.maps i).ratio)

lemma jointRatio_lt_one : ifs.jointRatio < 1 := by
  rw [jointRatio, Finset.sup'_lt_iff]
  exact fun i _ => (ifs.maps i).ratio_lt_one

lemma ratio_le_jointRatio (i : Fin N) :
    (ifs.maps i).ratio ≤ ifs.jointRatio :=
  Finset.le_sup' (f := fun j => (ifs.maps j).ratio) (Finset.mem_univ i)

/-- The Hutchinson operator `F(K) = ⋃ᵢ fᵢ(K)` on non-empty compact subsets. -/
noncomputable def hutchinson (K : NonemptyCompacts X) : NonemptyCompacts X where
  carrier := ⋃ i, (ifs.maps i).toFun '' (K : Set X)
  isCompact' :=
    isCompact_iUnion (fun i => K.isCompact.image (ifs.maps i).continuous)
  nonempty' := by
    have ⟨i⟩ : Nonempty (Fin N) := ⟨⟨0, Nat.pos_of_ne_zero (NeZero.ne N)⟩⟩
    obtain ⟨x, hx⟩ := K.nonempty
    exact ⟨(ifs.maps i).toFun x, mem_iUnion.mpr ⟨i, mem_image_of_mem _ hx⟩⟩

@[simp]
lemma coe_hutchinson (K : NonemptyCompacts X) :
    ((ifs.hutchinson K : NonemptyCompacts X) : Set X) =
      ⋃ i, (ifs.maps i).toFun '' (K : Set X) := rfl

private lemma edist_eq_hausdorffEDist (K L : NonemptyCompacts X) :
    edist K L = hausdorffEDist (K : Set X) (L : Set X) := rfl

/-- The Hutchinson operator is a contraction with the joint ratio. -/
theorem hutchinson_contracting :
    ContractingWith ifs.jointRatio (ifs.hutchinson) := by
  refine ⟨ifs.jointRatio_lt_one, fun K L => ?_⟩
  rw [edist_eq_hausdorffEDist, edist_eq_hausdorffEDist, coe_hutchinson, coe_hutchinson]
  refine hausdorffEDist_iUnion_le.trans ?_
  refine iSup_le (fun i => ?_)
  calc hausdorffEDist ((ifs.maps i).toFun '' (K : Set X))
                       ((ifs.maps i).toFun '' (L : Set X))
      ≤ ((ifs.maps i).ratio : ℝ≥0∞) * hausdorffEDist (K : Set X) (L : Set X) :=
          hausdorffEDist_image_le_mul (ifs.maps i).lipschitz K.nonempty L.nonempty
    _ ≤ (ifs.jointRatio : ℝ≥0∞) * hausdorffEDist (K : Set X) (L : Set X) := by
          gcongr
          exact_mod_cast ifs.ratio_le_jointRatio i

/-- The Hausdorff edistance between two non-empty compact subsets of a
metric space is finite. -/
private lemma compactsEDist_ne_top (K L : NonemptyCompacts X) :
    edist K L ≠ ∞ := by
  show hausdorffEDist (K : Set X) (L : Set X) ≠ ∞
  exact hausdorffEDist_ne_top_of_nonempty_of_bounded
    K.nonempty L.nonempty K.isCompact.isBounded L.isCompact.isBounded

/-- **Hutchinson's theorem.** Every IFS on a complete (non-empty) metric
space has a unique non-empty compact attractor. -/
theorem attractor_exists_unique [Nonempty X] :
    ∃! A : NonemptyCompacts X, ifs.hutchinson A = A := by
  let p : X := Classical.arbitrary X
  let K₀ : NonemptyCompacts X :=
    ⟨⟨{p}, isCompact_singleton⟩, singleton_nonempty p⟩
  obtain ⟨A, hAfix, _⟩ :=
    ifs.hutchinson_contracting.exists_fixedPoint K₀ (compactsEDist_ne_top _ _)
  refine ⟨A, hAfix, fun B hBfix => ?_⟩
  rcases ifs.hutchinson_contracting.eq_or_edist_eq_top_of_fixedPoints hBfix hAfix with h | h
  · exact h
  · exact (compactsEDist_ne_top B A h).elim

/-- Set-level statement: there is a unique non-empty compact `A ⊆ X` with
`A = ⋃ᵢ fᵢ(A)`. -/
theorem attractor_set_exists_unique [Nonempty X] :
    ∃! A : Set X, A.Nonempty ∧ IsCompact A ∧
      A = ⋃ i, (ifs.maps i).toFun '' A := by
  obtain ⟨A, hA, hUniq⟩ := ifs.attractor_exists_unique
  refine ⟨(A : Set X), ⟨A.nonempty, A.isCompact, ?_⟩, ?_⟩
  · have h := congrArg (fun B : NonemptyCompacts X => (B : Set X)) hA
    simpa [coe_hutchinson] using h.symm
  · rintro B ⟨hne, hcomp, hfix⟩
    have hCompacts :
        ifs.hutchinson ⟨⟨B, hcomp⟩, hne⟩ = ⟨⟨B, hcomp⟩, hne⟩ := by
      apply NonemptyCompacts.ext
      simpa [coe_hutchinson] using hfix.symm
    have hEq := hUniq _ hCompacts
    exact congrArg (fun B : NonemptyCompacts X => (B : Set X)) hEq

end IFS

end IFS

end MoonMath
```

Two ingredients carry the proof:

1. **`hausdorffEDist_image_le_mul`** — a Lipschitz map with constant `K` shrinks Hausdorff edistance by `K`. Proved from scratch (Mathlib v4.28.0 has the union bound `hausdorffEDist_iUnion_le` but no Lipschitz-image specialisation, so we discharge the `K = 0` and `K ≠ 0` cases manually).
2. **`compactsEDist_ne_top`** — non-empty compact subsets of a metric space sit at finite Hausdorff edistance (Mathlib's `hausdorffEDist_ne_top_of_nonempty_of_bounded`). This lets us turn the EMetric Banach theorem (`exists_fixedPoint` + `eq_or_edist_eq_top_of_fixedPoints`) into the genuine uniqueness statement, side-stepping the absence of a `MetricSpace (NonemptyCompacts X)` instance in Mathlib.
