+++
title = "Carmichael Numbers"
description = "Carmichael numbers are composite integers n satisfying a^n ≡ a (mod n) for all a, fooling Fermat's primality test"
weight = 220
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "n \\text{ is Carmichael} \\iff n \\text{ is composite and } a^n \\equiv a \\pmod{n} \\text{ for all } a"
prerequisites = ["fermats-little-theorem", "fundamental-theorem-arithmetic"]
lean4_status = "complete"
+++

## Statement

A **Carmichael number** is a composite integer $n > 1$ such that $a^n \equiv a \pmod{n}$ for every integer $a$ — mimicking [[Fermat's Little Theorem]] despite not being prime.

**Korselt's criterion** characterises them: $n$ is a Carmichael number if and only if

1. $n$ is squarefree (no prime square divides $n$), and
2. $(p - 1) \mid (n - 1)$ for every prime $p \mid n$.

The smallest Carmichael number is $561 = 3 \times 11 \times 17$.

## Visualization

**Verification of Korselt's criterion for the four smallest Carmichael numbers:**

| $n$   | Factorisation       | Squarefree? | $(p-1) \mid (n-1)$? |
|--------|---------------------|-------------|----------------------|
| 561    | $3 \times 11 \times 17$ | Yes         | $2\mid 560$, $10\mid 560$, $16\mid 560$ — Yes |
| 1105   | $5 \times 13 \times 17$ | Yes         | $4\mid 1104$, $12\mid 1104$, $16\mid 1104$ — Yes |
| 1729   | $7 \times 13 \times 19$ | Yes         | $6\mid 1728$, $12\mid 1728$, $18\mid 1728$ — Yes |
| 2465   | $5 \times 17 \times 29$ | Yes         | $4\mid 2464$, $16\mid 2464$, $28\mid 2464$ — Yes |

Check for 561: $n - 1 = 560 = 2^4 \times 5 \times 7$.
- $p = 3$: $p - 1 = 2$, and $2 \mid 560$. Yes.
- $p = 11$: $p - 1 = 10$, and $10 \mid 560$ ($560 = 56 \times 10$). Yes.
- $p = 17$: $p - 1 = 16$, and $16 \mid 560$ ($560 = 35 \times 16$). Yes.

## Proof Sketch

1. If $n = p_1 \cdots p_k$ is squarefree and $(p_i - 1) \mid (n-1)$ for each $i$, then for any $a$: if $p_i \mid a$ then $p_i \mid a^n$; if $p_i \nmid a$ then $a^{p_i - 1} \equiv 1 \pmod{p_i}$ by [[Fermat's Little Theorem]], so $a^n = a \cdot (a^{p_i - 1})^{(n-1)/(p_i-1)} \equiv a \pmod{p_i}$.
2. By the [[Chinese Remainder Theorem]], $a^n \equiv a \pmod{n}$.
3. Conversely, if $n$ is not squarefree or some $(p_i - 1) \nmid (n-1)$, one can exhibit an $a$ with $a^n \not\equiv a$.
4. Korselt (1899) proved this characterisation; Robert Carmichael verified $561$ in 1910.

## Connections

Carmichael numbers demonstrate that Fermat's primality test (checking $a^{n-1} \equiv 1 \pmod{n}$) is not conclusive. The [[Fundamental Theorem of Arithmetic]] underpins Korselt's squarefree condition; the Chinese Remainder Theorem (see [[Chinese Remainder Theorem]]) patches the local conditions together.

## Lean4 Proof

```lean4
/-- 561 = 3 * 11 * 17 — the factorisation of the smallest Carmichael number. -/
theorem carmichael_561_factored : 561 = 3 * 11 * 17 := by norm_num

/-- 561 is squarefree: no prime squared divides it. -/
theorem carmichael_561_squarefree : Squarefree 561 := by decide
```
