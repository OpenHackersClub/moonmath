+++
title = "Jordan-Hölder Theorem"
description = "Any two composition series of a group have the same length and isomorphic composition factors (in some order)."
weight = 110
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "\\{0\\} \\subset H_1 \\subset \\cdots \\subset G \\text{ and } \\{0\\} \\subset K_1 \\subset \\cdots \\subset G \\text{ give the same factors}"
prerequisites = ["first-isomorphism-theorem", "sylow-theorems"]
lean4_status = "complete"
+++

## Statement

A **composition series** of a group $G$ is a subnormal series

$$\{e\} = G_n \triangleleft G_{n-1} \triangleleft \cdots \triangleleft G_1 \triangleleft G_0 = G$$

where each quotient $G_i/G_{i+1}$ is a **simple group** (no normal subgroups except $\{e\}$ and itself).

**Jordan–Hölder Theorem.** If $G$ has two composition series, they have the same length $n$ (the **composition length**), and the multisets of composition factors $\{G_i/G_{i+1}\}$ are the same up to isomorphism and reordering.

## Visualization

**Two composition series of $\mathbb{Z}/12$:**

Series A: $\{0\} \subset \mathbb{Z}/2 \subset \mathbb{Z}/6 \subset \mathbb{Z}/12$

| Step | Quotient | Simple? | Order |
|---|---|---|---|
| $\mathbb{Z}/2 / \{0\}$ | $\mathbb{Z}/2$ | Yes ($p$-group) | 2 |
| $\mathbb{Z}/6 / \mathbb{Z}/2$ | $\mathbb{Z}/3$ | Yes | 3 |
| $\mathbb{Z}/12 / \mathbb{Z}/6$ | $\mathbb{Z}/2$ | Yes | 2 |

Series B: $\{0\} \subset \mathbb{Z}/3 \subset \mathbb{Z}/6 \subset \mathbb{Z}/12$

| Step | Quotient | Simple? | Order |
|---|---|---|---|
| $\mathbb{Z}/3 / \{0\}$ | $\mathbb{Z}/3$ | Yes | 3 |
| $\mathbb{Z}/6 / \mathbb{Z}/3$ | $\mathbb{Z}/2$ | Yes | 2 |
| $\mathbb{Z}/12 / \mathbb{Z}/6$ | $\mathbb{Z}/2$ | Yes | 2 |

Both series have length 3 and factors $\{\mathbb{Z}/2, \mathbb{Z}/2, \mathbb{Z}/3\}$ — the Jordan–Hölder theorem guarantees this.

## Proof Sketch

1. **Existence.** Any finite group has a composition series (insert maximal normal subgroups greedily, terminate since $|G|$ is finite).
2. **Uniqueness (the hard part, by induction on length).** Given two series $\{G_i\}$ and $\{H_j\}$, compare the first steps $G_1 \triangleleft G$ and $H_1 \triangleleft G$. Apply the second isomorphism theorem to $G_1 H_1 / G_1 \cong H_1 / (G_1 \cap H_1)$ and a symmetric isomorphism, reducing to shorter series to which the inductive hypothesis applies.
3. **Simple groups are the atoms.** The composition factors are the "prime factors" of $G$ — a decomposition that cannot be refined further.

## Connections

The Jordan–Hölder theorem is the group analogue of unique prime factorization from the [[Fundamental Theorem of Arithmetic]]. Its proof pivots on the [[Second Isomorphism Theorem]] (the "diamond" isomorphism) to relate two adjacent composition series steps. The composition factors of finite groups are ultimately classified by the enormous Classification of Finite Simple Groups — see [[Sylow Theorems]] for a piece of that structure.

## Lean4 Proof

```lean4
/-- The Jordan-Hölder theorem: any two composition series with the same endpoints
    are equivalent (same length, isomorphic factors up to reordering).
    Mathlib: `CompositionSeries.jordan_holder` in `Mathlib.Order.JordanHolder`. -/
theorem jordan_holder_thm {X : Type*} [Lattice X] [JordanHolderLattice X]
    (s₁ s₂ : CompositionSeries X)
    (hb : s₁.head = s₂.head) (ht : s₁.last = s₂.last) :
    CompositionSeries.Equivalent s₁ s₂ :=
  CompositionSeries.jordan_holder s₁ s₂ hb ht
```
