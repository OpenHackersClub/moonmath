+++
title = "Gram–Schmidt Orthogonalization"
description = "Any linearly independent sequence can be transformed into an orthogonal sequence spanning the same subspace."
weight = 110
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "e_k = f_k - \\sum_{j < k} \\frac{\\langle f_k, e_j\\rangle}{\\langle e_j, e_j\\rangle} e_j"
prerequisites = ["rank-nullity", "spectral-theorem"]
lean4_status = "complete"
+++

## Statement

Let $E$ be an inner product space and $f_1, f_2, \ldots, f_n$ a finite sequence of vectors. The **Gram–Schmidt process** produces vectors $e_1, e_2, \ldots, e_n$ defined by

$$e_k = f_k - \sum_{j < k} \frac{\langle f_k, e_j \rangle}{\langle e_j, e_j \rangle} e_j$$

satisfying:

1. **Orthogonality:** $\langle e_i, e_j \rangle = 0$ for $i \ne j$.
2. **Span preservation:** $\mathrm{span}\{e_1, \ldots, e_k\} = \mathrm{span}\{f_1, \ldots, f_k\}$ for all $k$.

If the $f_i$ are linearly independent, no $e_k$ is zero and normalizing gives an **orthonormal basis**.

## Visualization

Orthogonalize $f_1 = (1,1,0)$, $f_2 = (1,0,1)$, $f_3 = (0,1,1)$ in $\mathbb{R}^3$:

```
Step 1: e1 = f1 = (1, 1, 0)
        ||e1||^2 = 2

Step 2: proj of f2 onto e1 = <f2,e1>/<e1,e1> * e1
                            = (1*1+0*1+1*0)/2 * (1,1,0)
                            = 1/2 * (1,1,0) = (1/2, 1/2, 0)
        e2 = f2 - proj = (1,0,1) - (1/2,1/2,0) = (1/2, -1/2, 1)
        ||e2||^2 = 1/4 + 1/4 + 1 = 3/2

Step 3: proj of f3 onto e1 = <f3,e1>/2 * e1
                            = (0+1+0)/2 * (1,1,0) = (1/2, 1/2, 0)
        proj of f3 onto e2 = <f3,e2>/(3/2) * e2
                            = (0+(-1/2)+1)/(3/2) * (1/2,-1/2,1)
                            = (1/2)/(3/2) * (1/2,-1/2,1)
                            = 1/3 * (1/2,-1/2,1) = (1/6,-1/6,1/3)
        e3 = f3 - (1/2,1/2,0) - (1/6,-1/6,1/3)
           = (0,1,1) - (4/6, 2/6, 2/6)
           = (-2/3, 2/3, 2/3)

Check: <e1,e2> = 1/2 - 1/2 + 0 = 0 ✓
       <e1,e3> = -2/3 + 2/3 + 0 = 0 ✓
       <e2,e3> = -1/3 - 1/3 + 2/3 = 0 ✓
```

## Proof Sketch

1. **Orthogonality by induction.** Assume $e_1, \ldots, e_{k-1}$ are pairwise orthogonal. For $i < k$: $\langle e_k, e_i \rangle = \langle f_k, e_i \rangle - \frac{\langle f_k, e_i \rangle}{\langle e_i, e_i \rangle} \langle e_i, e_i \rangle = 0$, since all other subtracted terms are orthogonal to $e_i$ by hypothesis.
2. **Span preservation.** Each $e_k$ differs from $f_k$ by a linear combination of $e_1, \ldots, e_{k-1}$, which by induction equals a combination of $f_1, \ldots, f_{k-1}$. So $e_k \in \mathrm{span}\{f_1, \ldots, f_k\}$ and $f_k = e_k + (\text{span of earlier})$, giving equality of spans.
3. **Nonzero output.** If the $f_i$ are linearly independent, $f_k \notin \mathrm{span}\{f_1, \ldots, f_{k-1}\} = \mathrm{span}\{e_1, \ldots, e_{k-1}\}$, so $e_k \ne 0$.

## Connections

Gram–Schmidt is the foundational step in constructing [[QR Decomposition]] and in proving the [[Spectral Theorem]] (where an eigenbasis can always be orthonormalized). The span-preservation property mirrors the row-span invariance of [[Rank–Nullity Theorem]].

## Lean4 Proof

```lean4
-- Mathlib: Mathlib.Analysis.InnerProductSpace.GramSchmidtOrtho
-- Two key theorems about `gramSchmidt`:
--   gramSchmidt_orthogonal : pairwise orthogonal output
--   span_gramSchmidt       : output spans the same subspace as input

open inner_product_geometry in
theorem gs_pairwise_orthogonal
    (ι : Type*) [LinearOrder ι] [LocallyFiniteOrder ι]
    (E : Type*) [RCLike ℝ] [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
    (f : ι → E) :
    Pairwise fun a b => @inner ℝ E _ (gramSchmidt ℝ f a) (gramSchmidt ℝ f b) = 0 :=
  gramSchmidt_pairwise_orthogonal ℝ f

theorem gs_span_eq
    (ι : Type*) [LinearOrder ι] [LocallyFiniteOrder ι]
    (E : Type*) [RCLike ℝ] [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
    (f : ι → E) :
    Submodule.span ℝ (Set.range (gramSchmidt ℝ f)) =
    Submodule.span ℝ (Set.range f) :=
  span_gramSchmidt ℝ f
```