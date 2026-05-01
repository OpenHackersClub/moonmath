+++
title = "Correspondence Theorem"
description = "Subgroups of G/N are in bijection with subgroups of G containing N, preserving the lattice"
weight = 150
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "\\{H \\leq G : N \\leq H\\} \\;\\ \\longleftrightarrow\\ \\; \\{\\bar{H} \\leq G/N\\}"
prerequisites = ["first-isomorphism-theorem", "third-isomorphism-theorem"]
lean4_status = "complete"
+++

## Statement

Let $N \unlhd G$ be a normal subgroup and $\pi : G \to G/N$ the canonical projection. There is an inclusion-preserving bijection

$$\bigl\{H \leq G \;\mid\; N \leq H\bigr\} \;\xleftrightarrow{\;\sim\;}\; \bigl\{\bar{H} \leq G/N\bigr\},$$

given by $H \mapsto H/N = \pi(H)$ with inverse $\bar{H} \mapsto \pi^{-1}(\bar{H})$. This bijection preserves and reflects:

- Inclusion ($H_1 \leq H_2 \Leftrightarrow H_1/N \leq H_2/N$)
- Normality ($H \unlhd G \Leftrightarrow H/N \unlhd G/N$)
- Index ($[G : H] = [G/N : H/N]$)

## Visualization

Take $G = \mathbb{Z}/12\mathbb{Z}$ and $N = \langle 4 \rangle = \{0, 4, 8\}$, so $G/N \cong \mathbb{Z}/4\mathbb{Z}$.

```
Subgroups of ℤ/12ℤ containing {0,4,8}      Subgroups of ℤ/4ℤ
─────────────────────────────────────       ───────────────────
ℤ/12ℤ  =  {0,1,2,...,11}     [index 1]  ←→  ℤ/4ℤ   [index 1]
  ⟨2⟩  =  {0,2,4,6,8,10}    [index 2]  ←→  ⟨2⟩    [index 2]
  ⟨4⟩  =  {0,4,8}           [index 4]  ←→  {0}     [index 4]

(subgroup ⟨3⟩ = {0,3,6,9} does NOT contain {0,4,8} — excluded)
```

The lattice on the left (restricted to subgroups containing $N$) is isomorphic to the full lattice on the right. Inclusion is preserved: $\{0,4,8\} \leq \{0,2,4,6,8,10\} \leq \mathbb{Z}/12\mathbb{Z}$ maps to $\{0\} \leq \langle 2 \rangle \leq \mathbb{Z}/4\mathbb{Z}$.

## Proof Sketch

1. Define $\Phi : H \mapsto \pi(H) = H/N$ and $\Psi : \bar{H} \mapsto \pi^{-1}(\bar{H})$.
2. Show $\Phi \circ \Psi = \text{id}$: For any $\bar{H} \leq G/N$, the preimage $\pi^{-1}(\bar{H})$ contains $N$ (since $\pi^{-1}(\{e\}) = N$), and $\pi(\pi^{-1}(\bar{H})) = \bar{H}$ because $\pi$ is surjective.
3. Show $\Psi \circ \Phi = \text{id}$: For $H \geq N$, the set $\pi^{-1}(\pi(H)) = HN = H$ (because $N \leq H$).
4. Inclusion-preservation follows from $\pi$ being a group homomorphism.

## Connections

The Correspondence Theorem is the lattice-theoretic upgrade of the [[First Isomorphism Theorem]] and implies the [[Third Isomorphism Theorem]] as a special case (taking two nested subgroups). It is central to the [[Fundamental Theorem of Galois Theory]], where subgroups of the Galois group correspond to intermediate fields — the Correspondence Theorem is exactly the tool that makes this bijection order-reversing.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.QuotientGroup.Basic

/-- **Correspondence Theorem**: for N ≤ H, the image of H under the quotient map
    satisfies H = comap ∘ map, i.e. the comap-map round-trip is the identity
    on subgroups containing N.
    Mathlib: `Subgroup.comap_map_eq` applied to the quotient map. -/
theorem correspondence_comap_map_eq
    {G : Type*} [Group G] (N : Subgroup G) [N.Normal] (H : Subgroup G) (hNH : N ≤ H) :
    (H.map (QuotientGroup.mk' N)).comap (QuotientGroup.mk' N) = H := by
  rw [Subgroup.comap_map_eq, QuotientGroup.ker_mk']
  exact sup_eq_right.mpr hNH
```
