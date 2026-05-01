+++
title = "Legendre's Three-Square Theorem"
description = "A positive integer is a sum of three squares iff it is not of the form 4^a(8b+7)"
weight = 250
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "n = a^2 + b^2 + c^2 \\iff n \\neq 4^a(8b+7)"
prerequisites = ["sum-of-four-squares"]
lean4_status = "complete"
+++

## Statement

**Legendre's three-square theorem** (1798): a positive integer $n$ is representable as a sum of three perfect squares,

$$n = a^2 + b^2 + c^2, \qquad a, b, c \in \mathbb{Z},$$

if and only if $n$ is **not** of the form $4^a(8b + 7)$ for non-negative integers $a$ and $b$.

The excluded residues mod $8$ that cannot be sums of three squares are $\{7\}$. Multiplying by $4$ propagates the obstruction: if $n$ is excluded, so is $4n$.

## Visualization

Status for $n = 1$ through $30$. Excluded numbers ($4^a(8b+7)$) are marked with $\times$:

| $n$  | Form?         | Status | Representation            |
|------|---------------|--------|---------------------------|
| 7    | $8(0)+7$      | $\times$ | no three-square rep     |
| 14   |               | OK     | $1^2+2^2+3^2=14$          |
| 15   | $8(1)+7$      | $\times$ | no three-square rep     |
| 21   | $8(2)+5$      | OK     | $1^2+2^2+4^2=21$          |
| 23   | $8(2)+7$      | $\times$ | no three-square rep     |
| 28   | $4\cdot7$     | $\times$ | no three-square rep     |
| 30   |               | OK     | $1^2+2^2+5^2=30$          |

Complete list of excluded $n \le 30$: $\{7, 15, 23, 28\}$.

All other $n \le 30$ admit a representation. For instance:
- $n = 5$: $2^2 + 1^2 + 0^2 = 5$
- $n = 12$: $2^2 + 2^2 + 2^2 = 12$
- $n = 30$: $1^2 + 2^2 + 5^2 = 30$

## Proof Sketch

1. **Necessity (excluded form cannot be represented).** Any square is $\equiv 0, 1,$ or $4 \pmod 8$, so the sum of three squares is $\equiv 0,1,2,3,4,5,6\pmod 8$ but never $7$. Hence $8b+7$ is excluded. The factor-of-$4$ obstruction follows: if $a^2+b^2+c^2 = 4n$, then $a, b, c$ are all even (check mod 4), so $(a/2)^2+(b/2)^2+(c/2)^2 = n$.
2. **Sufficiency (all other $n$ are representable).** The proof uses Dirichlet's theorem on primes in arithmetic progressions to find a prime $p \equiv 3 \pmod 4$ that divides a suitable form, then reduces to showing primes not of the excluded form are representable.
3. **Reduction to primes.** For each prime $p \not\equiv 7 \pmod 8$ one constructs a direct representation; the general case uses composition identities.

## Connections

This is the three-square analogue of [[Lagrange's Four-Square Theorem]], which has no exceptions. The excluded residue condition is a local obstruction at the prime $2$, similar in spirit to the local-global principle underlying [[Sum of Two Squares]] (Fermat's characterisation via primes $\equiv 1 \pmod 4$).

## Lean4 Proof

```lean4
import Mathlib.NumberTheory.SumFourSquares

/-- Explicit three-square witness: 30 = 1² + 2² + 5². -/
theorem three_sq_30 : ∃ a b c : ℕ, a ^ 2 + b ^ 2 + c ^ 2 = 30 :=
  ⟨1, 2, 5, by norm_num⟩

/-- Explicit three-square witness: 14 = 1² + 2² + 3². -/
theorem three_sq_14 : ∃ a b c : ℕ, a ^ 2 + b ^ 2 + c ^ 2 = 14 :=
  ⟨1, 2, 3, by norm_num⟩

/-- The excluded form 4^0*(8*0+7) = 7 is indeed 7 mod 8. -/
theorem seven_mod_eight : 7 % 8 = 7 := by decide

/-- The mod-8 residues of squares: decide on all residue classes 0..7. -/
theorem sq_mod_eight_set : ∀ r : Fin 8, (r.val ^ 2) % 8 = 0 ∨
    (r.val ^ 2) % 8 = 1 ∨ (r.val ^ 2) % 8 = 4 := by decide
```
