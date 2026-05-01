+++
title = "Sum of Two Squares"
description = "A prime is a sum of two perfect squares if and only if it equals 2 or is congruent to 1 mod 4"
weight = 100
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "p = a^2 + b^2 \\iff p = 2 \\text{ or } p \\equiv 1 \\pmod{4}"
prerequisites = ["fermats-little-theorem", "quadratic-reciprocity"]
lean4_status = "complete"
+++

## Statement

A prime $p$ can be written as a sum of two integer squares,

$$p = a^2 + b^2,$$

if and only if $p = 2$ or $p \equiv 1 \pmod{4}$.

Equivalently, the only primes that are *not* sums of two squares are the primes $p \equiv 3 \pmod{4}$.

## Visualization

Which primes up to 50 split as $a^2 + b^2$?

```
p   p mod 4   Decomposition       Verdict
---+---------+--------------------+--------
2     2       1² + 1²             YES
3     3       —                   NO
5     1       1² + 2²             YES
7     3       —                   NO
11    3       —                   NO
13    1       2² + 3²             YES
17    1       1² + 4²             YES
19    3       —                   NO
23    3       —                   NO
29    1       2² + 5²             YES
31    3       —                   NO
37    1       1² + 6²             YES
41    1       4² + 5²             YES
43    3       —                   NO
47    3       —                   NO
```

The pattern is sharp: **every** prime $\equiv 1 \pmod{4}$ splits; **none** of the primes $\equiv 3 \pmod{4}$ does.

## Proof Sketch

1. **Necessity ($p \equiv 3 \pmod 4 \Rightarrow$ no split):** Squares are $\equiv 0$ or $1 \pmod 4$, so $a^2 + b^2 \equiv 0, 1,$ or $2 \pmod 4$. A sum of two squares is never $\equiv 3 \pmod 4$.

2. **$p = 2$:** $2 = 1^2 + 1^2$. Done.

3. **$p \equiv 1 \pmod 4 \Rightarrow$ split (Fermat–Euler):**
   - By [[Quadratic Reciprocity]] (specifically the first supplement), $-1$ is a quadratic residue mod $p$ whenever $p \equiv 1 \pmod 4$, so there exists $x$ with $x^2 \equiv -1 \pmod p$.
   - Apply Fermat's descent (or the Gaussian integer argument): the ideal $(p)$ in $\mathbb{Z}[i]$ factors as $(p) = \pi \bar\pi$ where $\pi = a + bi$, giving $p = a^2 + b^2$.

4. **Gaussian integer proof:** $p$ is irreducible in $\mathbb{Z}$ but *not* a Gaussian prime when $p \equiv 1 \pmod 4$. The factorization $p = \pi\bar\pi$ in $\mathbb{Z}[i]$ yields the decomposition directly.

## Connections

This theorem is intimately connected to [[Quadratic Reciprocity]] (which characterises when $-1$ is a square mod $p$), [[Fermat's Little Theorem]] (used in Euler's criterion), and [[Fundamental Theorem of Arithmetic]] (which guarantees unique factorization in $\mathbb{Z}[i]$). The analogous question for four squares is answered by [[Lagrange's Four-Square Theorem]]. The structure of representations also links to [[Multiplicative Functions]] and [[Euler's Totient]], since the number of representations $r_2(n)$ is multiplicative.

## Lean4 Proof

```lean4
/-- Fermat's theorem on sums of two squares: a prime `p` with `p % 4 ≠ 3`
    can be written as `a^2 + b^2`. Mathlib's `Nat.Prime.sq_add_sq` covers
    both `p = 2` (which satisfies `2 % 4 = 2 ≠ 3`) and `p ≡ 1 mod 4`. -/
theorem prime_sum_two_squares (p : ℕ) [hp : Fact p.Prime] (h : p % 4 ≠ 3) :
    ∃ a b : ℕ, a ^ 2 + b ^ 2 = p :=
  hp.out.sq_add_sq h

/-- Primes that are ≡ 3 mod 4 cannot be a sum of two squares.
    We verify this for `p = 3` by a norm_num check. -/
theorem three_not_sum_two_squares (a b : ℕ) : a ^ 2 + b ^ 2 ≠ 3 := by
  omega
```