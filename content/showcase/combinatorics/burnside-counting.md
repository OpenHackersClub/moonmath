+++
title = "Burnside Counting"
description = "The number of orbits of a group action equals the average number of fixed points: |X/G| = (1/|G|) sum |X^g|."
weight = 110
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "|X/G| = \\frac{1}{|G|}\\sum_{g \\in G}|X^g|"
prerequisites = ["pigeonhole"]
lean4_status = "complete"
+++

## Statement

Let a finite group $G$ act on a finite set $X$. For each $g \in G$, let $X^g = \{x \in X : g \cdot x = x\}$ be the fixed-point set of $g$. Then the number of orbits $|X/G|$ satisfies:

$$|X/G| = \frac{1}{|G|} \sum_{g \in G} |X^g|$$

This is **Burnside's lemma** (also attributed to Cauchy and Frobenius). It converts a counting-orbits problem into averaging fixed points, which is often much easier to compute.

## Visualization

**Counting distinct necklaces of 3 beads, 2 colors (red/blue), under rotations.**

The group $C_3 = \{r^0, r^1, r^2\}$ (rotations by $0^\circ, 120^\circ, 240^\circ$) acts on the $2^3 = 8$ colorings:

| Coloring (positions 0,1,2) | Fixed by $r^0$? | Fixed by $r^1$? | Fixed by $r^2$? |
|---|---|---|---|
| RRR | yes | yes | yes |
| RRB | yes | no | no |
| RBR | yes | no | no |
| RBB | yes | no | no |
| BRR | yes | no | no |
| BRB | yes | no | no |
| BBR | yes | no | no |
| BBB | yes | yes | yes |

Fixed-point counts: $|X^{r^0}| = 8$, $|X^{r^1}| = 2$, $|X^{r^2}| = 2$.

Number of orbits $= \frac{1}{3}(8 + 2 + 2) = \frac{12}{3} = 4$.

The 4 distinct necklaces: all-red, all-blue, two-red-one-blue, one-red-two-blue.

## Proof Sketch

1. **Double-count pairs.** Count ordered pairs $(g, x)$ with $g \cdot x = x$.
2. **Sum over $g$:** each $g$ contributes $|X^g|$ pairs, giving $\sum_{g} |X^g|$.
3. **Sum over $x$:** each $x$ contributes $|\text{Stab}(x)|$ (its stabilizer size) pairs.
4. **Orbit-stabilizer theorem.** $|\text{Stab}(x)| = |G| / |\text{Orb}(x)|$.
5. **Sum over orbits.** Group elements by orbit: each orbit $O$ contributes $|G|/|O| \cdot |O| = |G|$ to the total. With $k$ orbits, the total is $k \cdot |G|$.
6. **Conclude.** $\sum_g |X^g| = k \cdot |G|$, so $k = \frac{1}{|G|}\sum_g |X^g|$.

## Connections

Burnside's lemma is the entry point to [[Polya Enumeration]], which refines it using cycle index polynomials to count colorings by weight. It also connects to [[Lagrange's Theorem]] (via the orbit-stabilizer theorem) and to [[Cayley's Theorem]] (group actions on themselves).

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.GroupAction.Quotient

/-- Burnside's lemma in Mathlib:
    sum_{g in G} |fixedBy g| = |orbits| * |G|.
    This is `MulAction.sum_card_fixedBy_eq_card_orbits_mul_card_group`. -/
theorem burnside_lemma_alias
    {G : Type*} [Group G] [Fintype G]
    {X : Type*} [MulAction G X] [Fintype X]
    [Fintype (MulAction.orbitRel.Quotient G X)]
    [∀ g : G, Fintype (MulAction.fixedBy X g)] :
    ∑ g : G, Fintype.card (MulAction.fixedBy X g) =
      Fintype.card (MulAction.orbitRel.Quotient G X) * Fintype.card G :=
  MulAction.sum_card_fixedBy_eq_card_orbits_mul_card_group G X

/-- For 3-bead 2-color necklaces under C_3, we get 4 orbits:
    (8 + 2 + 2) / 3 = 4. -/
theorem necklace_3_2 : (8 + 2 + 2) / 3 = 4 := by decide
```
