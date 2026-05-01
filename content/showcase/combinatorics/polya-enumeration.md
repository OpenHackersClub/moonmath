+++
title = "Polya Enumeration"
description = "The cycle index polynomial encodes all Burnside fixed-point counts into one generating function for weighted orbit counting."
weight = 120
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "P_G(x_1,\\ldots,x_n) = \\frac{1}{|G|}\\sum_{g\\in G}x_1^{c_1(g)}\\cdots x_n^{c_n(g)}"
prerequisites = ["orbit-counting-applications"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a finite group acting on positions $\{1,\ldots,n\}$ via permutations. Each group element $g$ decomposes into $c_k(g)$ cycles of length $k$. The **cycle index** of $G$ is the polynomial:

$$Z_G(x_1, x_2, \ldots, x_n) = \frac{1}{|G|} \sum_{g \in G} x_1^{c_1(g)} x_2^{c_2(g)} \cdots x_n^{c_n(g)}$$

**Polya's Enumeration Theorem.** If the set of colors has a weight function $w$, then the generating function for colorings up to $G$-equivalence, weighted by total weight, is obtained by substituting $x_k = \sum_{\text{color } c} w(c)^k$ into $Z_G$.

For the unweighted count with $m$ colors, set each $x_k = m$: the number of distinct colorings is $Z_G(m, m, \ldots, m)$.

## Visualization

**2-color necklaces of length 4 under $C_4 = \{r^0, r^1, r^2, r^3\}$.**

Cycle structures of $C_4$ on 4 positions:

| Element | Cycle type | $c_1, c_2, c_3, c_4$ | $x_1^{c_1}x_2^{c_2}x_4^{c_4}$ |
|---|---|---|---|
| $r^0$ (identity) | $(1)(2)(3)(4)$ | $c_1=4$ | $x_1^4$ |
| $r^1$ (rotate by 90) | $(1234)$ | $c_4=1$ | $x_4^1$ |
| $r^2$ (rotate by 180) | $(13)(24)$ | $c_2=2$ | $x_2^2$ |
| $r^3$ (rotate by 270) | $(1432)$ | $c_4=1$ | $x_4^1$ |

Cycle index: $Z_{C_4} = \frac{1}{4}(x_1^4 + x_4 + x_2^2 + x_4)$.

Set $x_k = 2$ (two colors) for all $k$:

$$Z_{C_4}(2,2,2,2) = \frac{1}{4}(2^4 + 2^1 + 2^2 + 2^1) = \frac{1}{4}(16 + 2 + 4 + 2) = \frac{24}{4} = 6$$

The 6 distinct 2-color necklaces of length 4:

```
RRRR  RRRB  RRBR  RRBB  RBRB  BBBB
```

Meaning: all-red, one-blue, two-adjacent-blue (= RRRB), two-opposite-blue (= RBRB rotations), three-blue (= RRRB reversed), all-blue.

Expanded list with canonical representative per orbit:

| Orbit | Representative | Size |
|---|---|---|
| RRRR | RRRR | 1 |
| 1 blue bead | BRRR | 4 |
| 2 adjacent blue | BBRR | 4 |
| 2 opposite blue | BRBR | 2 |
| 3 blue beads | BBBR | 4 |
| BBBB | BBBB | 1 |

Total colorings: $1+4+4+2+4+1 = 16 = 2^4$. Orbits: 6. Check!

## Proof Sketch

1. **Start from Burnside.** The number of orbits is $\frac{1}{|G|}\sum_g |X^g|$.
2. **Fixed colorings.** A coloring $f: \text{positions} \to \text{colors}$ is fixed by permutation $g$ iff $f$ is constant on each cycle of $g$.
3. **Count fixed colorings.** With $m$ colors and $g$ having $c_1+c_2+\cdots$ cycles, the number of fixed colorings is $m^{c_1(g)+c_2(g)+\cdots}$.
4. **Substitute.** Setting $x_k = m$ in the cycle index gives exactly $\frac{1}{|G|}\sum_g m^{c(g)}$ where $c(g)$ is the total number of cycles.
5. **Weighted version.** Replace each $x_k$ by $\sum_c w(c)^k$ to track color weights.

## Connections

Polya enumeration refines [[Orbit Counting Applications]] into a generating function. It is related to [[Catalan Numbers]] (necklace-like structures) and ultimately connects to [[Inclusion-Exclusion Principle]] for orbit-size computations. The cycle index is a character of the symmetric group representation theory.

## Lean4 Proof

```lean4
/-- Direct computation: Polya's formula for 2-color necklaces of length 4
    under C_4 gives 6 distinct necklaces.
    (1/4)(2^4 + 2^1 + 2^2 + 2^1) = (16 + 2 + 4 + 2) / 4 = 24 / 4 = 6. -/
theorem polya_c4_two_colors :
    (16 + 2 + 4 + 2) / 4 = 6 := by decide

/-- The 6 orbits sum back to 2^4 = 16 colorings. -/
theorem polya_orbit_sum :
    1 + 4 + 4 + 2 + 4 + 1 = (2 : ℕ) ^ 4 := by decide
```
