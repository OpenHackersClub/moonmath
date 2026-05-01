+++
title = "Hamming Bound"
description = "A binary code correcting t errors satisfies |C| * sum_{i=0}^{t} C(n,i) <= 2^n; the (7,4) Hamming code meets it"
weight = 154
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "|C| \\cdot \\sum_{i=0}^{t} \\binom{n}{i} \\le 2^n"
prerequisites = ["kraft-inequality"]
lean4_status = "complete"
+++

## Statement

A binary code $C \subseteq \{0,1\}^n$ can **correct $t$ errors** if distinct codewords have Hamming distance at least $2t + 1$. The **Hamming (sphere-packing) bound** is

$$|C| \cdot \sum_{i=0}^{t} \binom{n}{i} \le 2^n.$$

A code meeting this bound with equality is called **perfect**. The $[7, 4, 3]$ binary Hamming code corrects $t = 1$ error and is perfect:

$$|C| = 2^4 = 16, \quad \sum_{i=0}^{1}\binom{7}{i} = 1 + 7 = 8, \quad 16 \cdot 8 = 128 = 2^7.$$

## Visualization

The $[7, 4, 3]$ Hamming code generator matrix $G$ (systematic form):

```
G = [ 1 0 0 0 | 1 1 0 ]
    [ 0 1 0 0 | 1 0 1 ]
    [ 0 0 1 0 | 0 1 1 ]
    [ 0 0 0 1 | 1 1 1 ]
```

Parity-check matrix $H$ (3 rows, 7 columns — columns are all nonzero 3-bit strings):

```
H = [ 1 0 1 0 1 0 1 ]
    [ 0 1 1 0 0 1 1 ]
    [ 0 0 0 1 1 1 1 ]
```

Sphere-packing count: 16 codewords, each surrounded by a Hamming ball of radius 1 containing $1 + 7 = 8$ binary strings. Total covered: $16 \times 8 = 128 = 2^7$. The balls partition all of $\{0,1\}^7$ — perfect packing.

Correcting a single error: if received word $r = c + e_j$ (flip of bit $j$), compute syndrome $Hr^T = He_j^T = $ column $j$ of $H$. That column gives the error position directly.

## Proof Sketch

1. For each codeword $c \in C$, define its **Hamming ball** $B(c, t) = \{x \in \{0,1\}^n : d(c, x) \le t\}$. Each ball has size $\sum_{i=0}^{t}\binom{n}{i}$.
2. The minimum distance $\ge 2t+1$ implies balls around distinct codewords are disjoint.
3. All balls are subsets of $\{0,1\}^n$, which has $2^n$ elements.
4. By disjointness: $|C| \cdot |B(c, t)| \le 2^n$.

## Connections

The Hamming bound is the coding-theory analogue of the [[Pigeonhole Principle]]: pack balls in a space, count elements. The parity-check matrix construction links to [[Rank-Nullity Theorem]] over $\mathbb{F}_2$.

## Lean4 Proof

```lean4
/-- Verify the Hamming bound for the [7,4,3] binary code:
    16 codewords, t=1 errors, n=7 bits. -/
theorem hamming_bound_74 :
    (16 : ℕ) * (Nat.choose 7 0 + Nat.choose 7 1) = 2 ^ 7 := by
  native_decide
```
