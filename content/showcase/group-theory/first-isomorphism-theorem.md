+++
title = "First Isomorphism Theorem"
description = "The image of a homomorphism is isomorphic to the domain modulo the kernel"
weight = 40
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "G / \\ker(f) \\cong \\operatorname{im}(f)"
prerequisites = ["lagrange-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f : G \to H$ be a group homomorphism. Then:

$$G \,/\, \ker(f) \;\cong\; \operatorname{im}(f)$$

More precisely, the map $\bar{f} : G/\ker(f) \to H$ defined by $\bar{f}(g\ker f) = f(g)$ is a well-defined injective group homomorphism whose image equals $\operatorname{im}(f)$, giving the isomorphism.

## Visualization

Take the canonical surjection $f : \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z}$, $f(k) = k \bmod n$.

```
Kernel / Image Diagram (n = 6)

ℤ
│   f
│──────────────→  ℤ/6ℤ = {0,1,2,3,4,5}
│
ker(f) = 6ℤ = {..., -12, -6, 0, 6, 12, ...}
im(f)  = ℤ/6ℤ (all of the target; f is surjective)

Cosets of 6ℤ in ℤ:
  0 + 6ℤ = {..., -12, -6,  0,  6, 12, ...}  →  [0]
  1 + 6ℤ = {..., -11, -5,  1,  7, 13, ...}  →  [1]
  2 + 6ℤ = {..., -10, -4,  2,  8, 14, ...}  →  [2]
  3 + 6ℤ = {...,  -9, -3,  3,  9, 15, ...}  →  [3]
  4 + 6ℤ = {...,  -8, -2,  4, 10, 16, ...}  →  [4]
  5 + 6ℤ = {...,  -7, -1,  5, 11, 17, ...}  →  [5]
```

Six cosets, one for each element of $\mathbb{Z}/6\mathbb{Z}$. The First Isomorphism Theorem says the bijection between cosets and target elements is actually a group isomorphism:

$$\mathbb{Z} \,/\, 6\mathbb{Z} \;\xrightarrow{\;\cong\;}\; \mathbb{Z}/6\mathbb{Z}$$

Another example: $f : \mathbb{Z} \to \mathbb{Z}$ given by $f(k) = 2k$. Then $\ker(f) = \{0\}$ and $\operatorname{im}(f) = 2\mathbb{Z}$, so $\mathbb{Z}/\{0\} \cong 2\mathbb{Z}$, i.e.\ $\mathbb{Z} \cong 2\mathbb{Z}$ — both are infinite cyclic.

## Proof Sketch

1. **Well-defined**: if $g\ker f = g'\ker f$ then $g^{-1}g' \in \ker f$, so $f(g^{-1}g') = e$, so $f(g) = f(g')$.
2. **Homomorphism**: $\bar{f}(g\ker f \cdot h\ker f) = \bar{f}(gh\ker f) = f(gh) = f(g)f(h) = \bar{f}(g\ker f)\bar{f}(h\ker f)$.
3. **Injective**: $\bar{f}(g\ker f) = e \implies f(g) = e \implies g \in \ker f \implies g\ker f = \ker f$ (the identity coset).
4. **Surjective onto image**: by definition of $\operatorname{im}(f)$.

## Connections

The First Isomorphism Theorem is the engine of exact sequences, appearing throughout the [[Fundamental Theorem of Galois Theory]] (quotients of Galois groups correspond to sub-extensions). It quantifies [[Lagrange's Theorem]]: $|G/\ker f| = |\operatorname{im}(f)|$ gives $|\ker f| \cdot |\operatorname{im}(f)| = |G|$. In the proof of the [[Impossibility of the Quintic Formula]], surjective maps from $\text{Gal}(K/\mathbb{Q})$ onto quotients detect composition factors. [[Cayley's Theorem]] provides the embedding $G \hookrightarrow \text{Sym}(G)$ whose injectivity is a special case of this theorem ($\ker = 1$).

## Lean4 Proof

```lean4
/-- **First Isomorphism Theorem**: G / ker(f) ≃* range(f) for any group homomorphism f.
    Mathlib: `QuotientGroup.quotientKerEquivRange`
    in `Mathlib.GroupTheory.QuotientGroup.Basic`. -/
noncomputable def first_iso {G H : Type*} [Group G] [Group H] (f : G →* H) :
    G ⧸ f.ker ≃* f.range :=
  QuotientGroup.quotientKerEquivRange f
```
