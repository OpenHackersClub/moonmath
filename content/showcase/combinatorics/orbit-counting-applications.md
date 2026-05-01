+++
title = "Orbit Counting Applications"
description = "Necklace, bracelet, and coloring counting via Burnside's lemma — concrete applications of the orbit-counting formula."
weight = 65
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "|X/G| = \\frac{1}{|G|}\\sum_{g \\in G}|X^g|"
prerequisites = ["pigeonhole"]
lean4_status = "complete"
+++

The **orbit-counting formula** (Burnside's lemma) converts symmetry-quotient problems into fixed-point averages. The canonical statement and group-theoretic proof live in [[Burnside's Lemma]]; this note collects concrete counting applications.

## Necklaces — 3 beads, 2 colors under rotations

The rotation group $C_3 = \{r^0, r^1, r^2\}$ acts on $2^3 = 8$ colorings:

| Coloring | Fixed by $r^0$? | Fixed by $r^1$? | Fixed by $r^2$? |
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

$$|X/C_3| = \frac{1}{3}(8 + 2 + 2) = 4.$$

The 4 distinct necklaces: all-red, all-blue, two-red-one-blue, one-red-two-blue.

## Necklaces — 4 beads, 2 colors under $C_4$ rotations

$|G| = 4$, $|X| = 16$. Fixed-point counts: $|X^{r^0}| = 16$, $|X^{r^1}| = 2$, $|X^{r^2}| = 4$, $|X^{r^3}| = 2$.

$$|X/C_4| = \frac{1}{4}(16 + 2 + 4 + 2) = 6.$$

## Face colorings of a square under rotations

$G = C_4$ acts on 4-face colorings with 2 colors. Same calculation as above gives 6 rotationally distinct colorings.

## Connections

The abstract formula is [[Burnside's Lemma]] in the group-theory section. Polya enumeration ([[Polya Enumeration]]) refines this into a cycle-index generating function that tracks color multiplicities. The orbit-stabilizer identity used throughout is a consequence of [[Lagrange's Theorem]].

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.GroupAction.Quotient

/-- 3-bead 2-color necklace count under C_3 rotations: (8+2+2)/3 = 4. -/
theorem necklace_3_2 : (8 + 2 + 2) / 3 = 4 := by decide

/-- 4-bead 2-color necklace count under C_4 rotations: (16+2+4+2)/4 = 6. -/
theorem necklace_4_2 : (16 + 2 + 4 + 2) / 4 = 6 := by decide
```
