+++
title = "Second Isomorphism Theorem"
description = "For subgroups H and normal N of G, the quotient HN/N is isomorphic to H/(H ∩ N)."
weight = 130
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "HN/N \\cong H/(H \\cap N)"
prerequisites = ["first-isomorphism-theorem", "lagrange-theorem"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a group, $N \trianglelefteq G$ a normal subgroup, and $H \leq G$ any subgroup. Then:

1. $HN = \{hn \mid h \in H,\, n \in N\}$ is a subgroup of $G$, with $N \trianglelefteq HN$.
2. $H \cap N \trianglelefteq H$.
3. There is a group isomorphism

$$HN/N \cong H/(H \cap N).$$

This is sometimes called the **diamond isomorphism theorem** because of the lattice shape formed by $HN$, $H$, $N$, and $H \cap N$.

## Visualization

**Explicit example in $G = \mathbb{Z}$ (additive):**

Let $H = 2\mathbb{Z}$ (even integers), $N = 6\mathbb{Z}$ (multiples of 6).

Then $H + N = 2\mathbb{Z}$ (since $6\mathbb{Z} \subset 2\mathbb{Z}$, adding gives $2\mathbb{Z}$) and $H \cap N = \mathrm{lcm}(2,6)\mathbb{Z} = 6\mathbb{Z}$.

```
       2Z  (= HN)
      /    \
    2Z      6Z
      \    /
       6Z  (= H ∩ N)
```

Cosets of $6\mathbb{Z}$ in $2\mathbb{Z}$: $\{6\mathbb{Z},\ 2+6\mathbb{Z},\ 4+6\mathbb{Z}\}$ — three cosets.
Cosets of $H \cap N = 6\mathbb{Z}$ in $H = 2\mathbb{Z}$: same three cosets.

The isomorphism $2\mathbb{Z}/6\mathbb{Z} \cong 2\mathbb{Z}/6\mathbb{Z}$ is the identity here, but for a non-contained $H$ the identification is nontrivial.

**Second example: $G = S_4$, $H = \langle (12) \rangle \cong \mathbb{Z}/2$, $N = V_4 = \{e,(12)(34),(13)(24),(14)(23)\}$.**

- $HN = \langle (12) \rangle \cdot V_4$: has order $|H||N|/|H \cap N| = 2 \cdot 4 / 1 = 8$.
- $H \cap N = \{e\}$ (since $(12) \notin V_4$).
- Isomorphism: $HN/N \cong \mathbb{Z}/2/\{e\} \cong \mathbb{Z}/2$.

| Coset of $N$ in $HN$ | Elements |
|---|---|
| $N$ | $e, (12)(34), (13)(24), (14)(23)$ |
| $(12)N$ | $(12), (34), (1324), (1423)$ |

$|HN/N| = 2 = |H/(H \cap N)|$.

## Proof Sketch

1. Define the map $\phi : H \to HN/N$ by $\phi(h) = hN$.
2. $\phi$ is a group homomorphism (inherits from the quotient map $G \to G/N$).
3. $\phi$ is surjective: any coset $hnN = hN$ has preimage $h \in H$.
4. $\ker(\phi) = \{h \in H \mid hN = N\} = \{h \in H \mid h \in N\} = H \cap N$.
5. By the [[First Isomorphism Theorem]], $H/\ker(\phi) \cong \mathrm{im}(\phi) = HN/N$.

## Connections

The proof is a one-line application of the [[First Isomorphism Theorem]]. The same diamond pattern appears in the proof of the [[Jordan-Hölder Theorem]], where it is used to interchange adjacent factors of two composition series. The Third Isomorphism Theorem ($(G/N)/(H/N) \cong G/H$) is the other sibling, completing the trio.

## Lean4 Proof

```lean4
/-- Noether's Second Isomorphism Theorem: given a normal subgroup N ⊴ G and
    any subgroup H ≤ G, there is an isomorphism H/(H ∩ N) ≅ HN/N.
    Mathlib: `QuotientGroup.quotientInfEquivProdNormalQuotient`. -/
theorem second_isomorphism_theorem
    {G : Type*} [Group G] (H N : Subgroup G) [N.Normal] :
    H ⧸ N.subgroupOf H ≃* (H ⊔ N : Subgroup G) ⧸ N.subgroupOf (H ⊔ N) :=
  QuotientGroup.quotientInfEquivProdNormalQuotient H N
```
