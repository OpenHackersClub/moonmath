+++
title = "Catalan Numbers"
description = "The ubiquitous sequence counting balanced parentheses, binary trees, and Dyck paths"
weight = 30
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "C_n = \\frac{1}{n+1}\\binom{2n}{n}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

The $n$-th Catalan number is:

$$C_n = \frac{1}{n+1}\binom{2n}{n} = \binom{2n}{n} - \binom{2n}{n+1}$$

They satisfy the recursion:

$$C_0 = 1, \qquad C_{n+1} = \sum_{i=0}^{n} C_i \cdot C_{n-i}$$

## Visualization

**Values $C_0$ through $C_8$:**

| $n$ | 0 | 1 | 2 | 3 | 4  | 5   | 6    | 7     | 8      |
|-----|---|---|---|---|----|-----|------|-------|--------|
| $C_n$ | 1 | 1 | 2 | 5 | 14 | 42  | 132  | 429   | 1430   |

**Balanced parentheses for $n=3$ (all $C_3 = 5$ arrangements):**

```
((()))    (()())    (())()    ()(())    ()()()
```

**Dyck paths for $n=3$ — lattice paths from $(0,0)$ to $(6,0)$ that never go below the $x$-axis:**

```
Path 1: ((()))       U U U D D D
         /\
        /  \
       /    \___

Path 2: (()())       U U D U D D
         /\
        /  \/\

Path 3: (())()       U U D D U D
         /\
        /  \   /\

Path 4: ()(())       U D U U D D
        /\  /\
            /  \

Path 5: ()()()       U D U D U D
        /\ /\ /\
```

Each up-step $U$ corresponds to `(` and each down-step $D$ to `)`. The path must stay at or above height 0.

**Recursion unrolled for $C_4 = 14$:**

$$C_4 = C_0 C_3 + C_1 C_2 + C_2 C_1 + C_3 C_0 = 1\cdot5 + 1\cdot2 + 2\cdot1 + 5\cdot1 = 14$$

## Proof Sketch

**Ballot problem / reflection argument:** Count lattice paths from $(0,0)$ to $(2n, 0)$ with $+1$ and $-1$ steps that stay non-negative. Total paths: $\binom{2n}{n}$. Bad paths (those that go negative) biject with unrestricted paths from $(-2, 0)$ to $(2n, 0)$ via the reflection principle, numbering $\binom{2n}{n+1}$. So:

$$C_n = \binom{2n}{n} - \binom{2n}{n+1} = \frac{1}{n+1}\binom{2n}{n}$$

Mathlib's Catalan number satisfies `catalan_succ`: $C_{n+1} = \sum_{i : \text{Fin}(n+1)} C_i \cdot C_{n-i}$.

## Connections

- [[Pigeonhole Principle]] — the ballot problem uses a pigeonhole argument to count bad paths
- [[Inclusion-Exclusion Principle]] — bad paths counted via complementary counting
- [[Stirling Numbers]] — both sequences count partition-like structures; $C_n = S(2n, n)/(n+1)!$ (approximately)
- [[Bell Numbers]] — Bell numbers grow faster; $B_n \approx (n/\ln n)^n$ vs $C_n \approx 4^n / (n^{3/2}\sqrt{\pi})$
- [[Binomial Theorem]] — the closed form $C_n = \binom{2n}{n}/(n+1)$ directly uses binomial coefficients
- [[Lucas's Theorem]] — Lucas's theorem gives $C_p \equiv 2 \pmod{p}$ for prime $p$

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.Enumerative.Catalan

/-- The Catalan recursion: C(n+1) = ∑ᵢ C(i) * C(n-i).
    This is `catalan_succ` in Mathlib. -/
theorem catalan_recursion (n : ℕ) :
    catalan (n + 1) = ∑ i : Fin n.succ, catalan i * catalan (n - i) :=
  catalan_succ n

/-- Verify small values by computation. -/
theorem catalan_vals :
    catalan 0 = 1 ∧ catalan 1 = 1 ∧ catalan 2 = 2 ∧
    catalan 3 = 5 ∧ catalan 4 = 14 := by decide

/-- The number of Dyck words of semilength n equals C(n).
    This is `DyckWord.card_dyckWord_semilength_eq_catalan` in Mathlib. -/
theorem dyck_count_eq_catalan (n : ℕ) :
    Fintype.card { p : DyckWord // p.semilength = n } = catalan n :=
  DyckWord.card_dyckWord_semilength_eq_catalan n
```
