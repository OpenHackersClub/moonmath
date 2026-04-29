+++
title = "Iterated Function Systems"
description = "Constructing fractals via contractive affine transformations"
weight = 30
tags = ["interactive", "visualization", "fractal"]
latex = "A = \\bigcup_{i=1}^{N} f_i(A)"
prerequisites = ["hausdorff-dimension"]
lean4_status = "sorry"
+++

## Definition

An iterated function system (IFS) is a finite collection of contraction mappings $\{f_1, f_2, \ldots, f_N\}$ on a complete metric space. By the Banach fixed-point theorem (applied to the Hausdorff metric on compact sets), there exists a unique non-empty compact set $A$ — the **attractor** — satisfying:

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

The [[Hausdorff Dimension]] of an IFS attractor can be computed from the contraction ratios. The [[Mandelbrot Set]] is intimately connected to IFS theory through the dynamics of quadratic polynomials.

## Lean4 Proof

```lean4
/-- A contraction mapping on a metric space. -/
structure ContractionMap (X : Type*) [MetricSpace X] where
  toFun : X → X
  ratio : ℝ
  ratio_pos : 0 < ratio
  ratio_lt_one : ratio < 1
  contract : ∀ x y : X, dist (toFun x) (toFun y) ≤ ratio * dist x y

/-- An iterated function system is a finite collection of contractions. -/
structure IFS (X : Type*) [MetricSpace X] where
  maps : Fin N → ContractionMap X

/-- Existence and uniqueness of the attractor (Hutchinson, 1981):
    there is a unique non-empty compact set A with A = ⋃ᵢ fᵢ(A). -/
theorem attractor_exists_unique {X : Type*} [MetricSpace X] [CompleteSpace X]
    (ifs : IFS X) :
    ∃! A : Set X, A.Nonempty ∧ IsCompact A ∧
      A = ⋃ i, ifs.maps i |>.toFun '' A := by
  sorry -- Banach fixed-point theorem on the Hausdorff metric space of compact sets
```
