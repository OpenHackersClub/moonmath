+++
title = "Lagrange's Theorem"
description = "Subgroup order divides group order"
weight = 10
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "|H| \\;\\text{divides}\\; |G|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $G$ be a finite group and $H \leq G$ a subgroup. Then $|H|$ divides $|G|$, and the quotient $|G|/|H|$ equals the number of left cosets of $H$ in $G$ — the **index** $[G:H]$.

$$|G| = |H| \cdot [G : H]$$

## Visualization

Take $G = D_4$, the dihedral group of symmetries of a square, with $|D_4| = 8$.

```
D_4  (order 8)
 ├─ Z/4  (order 4, index 2)  — rotations {1, r, r², r³}
 ├─ Z/2 × Z/2  (order 4, index 2)  — {1, r², s, sr²} where s is a reflection
 ├─ Z/2  (order 2, index 4)  — {1, r²}
 ├─ Z/2  (order 2, index 4)  — {1, s}
 ├─ Z/2  (order 2, index 4)  — {1, sr}
 └─ {1}  (order 1, index 8)
```

Every subgroup order — 1, 2, 4 — divides $|D_4| = 8$. The cosets of $H = \{1, r²\}$ in $D_4$ are:

| Coset | Elements |
|-------|----------|
| $H$ | $\{1, r^2\}$ |
| $rH$ | $\{r, r^3\}$ |
| $sH$ | $\{s, sr^2\}$ |
| $srH$ | $\{sr, sr^3\}$ |

Four cosets, each of size 2: $8 = 2 \times 4$. Cosets partition $G$ — they are either identical or disjoint.

## Proof Sketch

1. Left cosets $gH$ and $g'H$ are either equal or disjoint (proven by showing $gH = g'H \iff g^{-1}g' \in H$).
2. Each coset has exactly $|H|$ elements (left multiplication by $g$ is a bijection $H \to gH$).
3. The cosets partition $G$ into $[G:H]$ equal pieces, so $|G| = |H| \cdot [G:H]$.

## Connections

Lagrange's theorem is the gateway to [[Sylow Theorems]], which ask the converse: for which divisors $p^k$ of $|G|$ does a subgroup of that order actually *exist*? It also underpins [[Cauchy's Theorem (Groups)]] (every prime divisor of $|G|$ yields an element of that order) and the counting in the [[First Isomorphism Theorem]]. More broadly it echoes through [[Fundamental Theorem of Galois Theory]] — subgroup indices equal field extension degrees — and appears in the proof of the [[Impossibility of the Quintic Formula]] via solvable group chains.

## Lean4 Proof

```lean4
/-- **Lagrange's Theorem**: the cardinality of any subgroup divides the cardinality
    of the ambient group.
    Mathlib: `Subgroup.card_subgroup_dvd_card` in `Mathlib.GroupTheory.Coset.Card`.
    Note: uses `Nat.card` (works for infinite groups too, giving 0 ∣ 0). -/
theorem lagrange {G : Type*} [Group G] (H : Subgroup G) :
    Nat.card H ∣ Nat.card G :=
  H.card_subgroup_dvd_card
```
