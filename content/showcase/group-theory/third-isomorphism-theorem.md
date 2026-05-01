+++
title = "Third Isomorphism Theorem"
description = "A quotient of quotients collapses to a single quotient: (G/N)/(H/N) ≅ G/H"
weight = 140
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "(G/N)/(H/N) \\cong G/H"
prerequisites = ["first-isomorphism-theorem", "lagrange-theorem"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a group with normal subgroups $N \unlhd H \unlhd G$. Then $H/N$ is normal in $G/N$, and there is a group isomorphism

$$\frac{G/N}{H/N} \;\cong\; G/H.$$

Equivalently: collapsing $N$ first and then $H/N$, or collapsing $H$ directly, yield the same quotient.

## Visualization

Take $G = \mathbb{Z}$, $H = 2\mathbb{Z}$, $N = 6\mathbb{Z}$, so $N \leq H \leq G$.

```
ℤ
│  mod 6
▼
ℤ/6ℤ = {0, 1, 2, 3, 4, 5}
│
│  H/N = 2ℤ/6ℤ = {0, 2, 4}  (index 2 inside ℤ/6ℤ)
▼
(ℤ/6ℤ) / (2ℤ/6ℤ)  ≅  ℤ/2ℤ = {0, 1}
```

Direct computation:

| Coset of $2\mathbb{Z}/6\mathbb{Z}$ in $\mathbb{Z}/6\mathbb{Z}$ | Elements | Maps to in $\mathbb{Z}/2\mathbb{Z}$ |
|---|---|---|
| $\{0, 2, 4\}$ | even residues mod 6 | $0$ |
| $\{1, 3, 5\}$ | odd residues mod 6 | $1$ |

The isomorphism sends a coset of $2\mathbb{Z}/6\mathbb{Z}$ to the corresponding element of $\mathbb{Z}/2\mathbb{Z}$. This equals the direct quotient $\mathbb{Z}/2\mathbb{Z} \cong \mathbb{Z}/2\mathbb{Z}$, confirming the theorem.

## Proof Sketch

1. Define $\pi : G/N \to G/H$ by $\pi(gN) = gH$. This is well-defined because $N \leq H$ implies $gN = g'N \Rightarrow g^{-1}g' \in N \leq H \Rightarrow gH = g'H$.
2. $\pi$ is a surjective group homomorphism.
3. Compute $\ker(\pi) = \{gN : gH = H\} = \{gN : g \in H\} = H/N$.
4. Apply the [[First Isomorphism Theorem]] to $\pi$: $(G/N)/\ker(\pi) \cong \operatorname{im}(\pi)$, giving $(G/N)/(H/N) \cong G/H$.

## Connections

This theorem is the third of Noether's trio, following the [[First Isomorphism Theorem]]. It is essential in the proof of the [[Fundamental Theorem of Galois Theory]], where the lattice of subgroups of $\text{Gal}(K/\mathbb{Q})$ corresponds to intermediate fields — and quotients of Galois groups correspond to quotients of extension degrees. It also arises naturally in the [[Sylow Theorems]] when passing between successive quotients.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.QuotientGroup.Basic

/-- **Third Isomorphism Theorem**: (G ⧸ N) ⧸ (H.map (mk' N)) ≃* G ⧸ H,
    whenever N ≤ H and both are normal.
    Mathlib: `QuotientGroup.quotientQuotientEquivQuotient`
    in `Mathlib.GroupTheory.QuotientGroup.Basic`. -/
noncomputable def third_iso
    {G : Type*} [Group G] (N : Subgroup G) [N.Normal]
    (H : Subgroup G) [H.Normal] (h : N ≤ H) :
    (G ⧸ N) ⧸ H.map (QuotientGroup.mk' N) ≃* G ⧸ H :=
  QuotientGroup.quotientQuotientEquivQuotient N H h
```
