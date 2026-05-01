+++
title = "Burnside's Lemma"
description = "The number of orbits equals the average size of fixed-point sets over the group."
weight = 60
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "|X/G| = \\frac{1}{|G|}\\sum_{g \\in G}|X^g|"
prerequisites = ["lagrange-theorem"]
lean4_status = "complete"
+++

## Statement

Let a finite group $G$ act on a finite set $X$. For each $g \in G$ write $X^g = \{x \in X \mid g \cdot x = x\}$ for the fixed-point set of $g$. Then the number of orbits is

$$|X/G| = \frac{1}{|G|}\sum_{g \in G}|X^g|.$$

This identity is also called the **Cauchy–Frobenius lemma**. It converts a hard orbit-counting problem into an averaging problem over group elements, which is often much easier.

## Visualization

**Example: 2-coloring the faces of a square under rotations.**

The rotation group of a square is $G = \{r^0, r^1, r^2, r^3\}$ (rotations by $0^\circ, 90^\circ, 180^\circ, 270^\circ$), so $|G| = 4$. We color each of the 4 faces black or white, giving $X = \{B,W\}^4$, $|X| = 16$.

| Rotation $g$ | Fixed colorings $|X^g|$ | Reason |
|---|---|---|
| $r^0$ (identity) | 16 | All 16 colorings fixed |
| $r^1$ (90°) | 2 | All 4 faces must match: BB or WW |
| $r^2$ (180°) | 4 | Opposite pairs must match: BBBB, BWBW, WBWB, WWWW |
| $r^3$ (270°) | 2 | Same as 90° by symmetry |

$$|X/G| = \frac{1}{4}(16 + 2 + 4 + 2) = \frac{24}{4} = 6.$$

There are exactly **6** rotationally distinct 2-colorings: BBBB, BBBW, BBWW (adjacent), BBWW (opposite), BWWW, WWWW.

## Proof Sketch

1. Count the set of pairs $\{(g,x) \mid g \cdot x = x\}$ two ways. Row-by-row over $g$ gives $\sum_g |X^g|$. Column-by-column over $x$ gives $\sum_x |\text{Stab}(x)|$.
2. By the orbit-stabilizer theorem, $|\text{Stab}(x)| = |G| / |\mathcal{O}(x)|$.
3. Therefore $\sum_x |\text{Stab}(x)| = |G| \cdot \sum_{\mathcal{O}} 1 = |G| \cdot |X/G|$, where the last sum counts one representative per orbit.
4. Equating the two counts gives $|G| \cdot |X/G| = \sum_g |X^g|$, and dividing through yields the lemma.

## Connections

The orbit-stabilizer identity used in step 2 is a consequence of [[Lagrange's Theorem]], which partitions any subgroup into cosets of equal size. The fixed-point averaging also underlies [[Cauchy's Theorem (Group)]], which counts elements of prime order.

## Lean4 Proof

```lean4
/-- Burnside's lemma: the number of orbits of a finite group action equals
    the average number of fixed points. Mathlib provides this via
    `MulAction.sum_card_fixedBy_eq_card_orbits_mul_card_group`. -/
theorem burnside_lemma {G X : Type*} [Group G] [Fintype G] [Fintype X]
    [DecidableEq X] [MulAction G X]
    [Fintype (MulAction.orbitRel.Quotient G X)] :
    Fintype.card G * Fintype.card (MulAction.orbitRel.Quotient G X) =
    ∑ g : G, Fintype.card {x : X // g • x = x} :=
  (MulAction.sum_card_fixedBy_eq_card_orbits_mul_card_group G X).symm
```
