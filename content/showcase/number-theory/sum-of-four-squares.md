+++
title = "Lagrange's Four-Square Theorem"
description = "Every natural number is the sum of four perfect squares"
weight = 240
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "n = a^2 + b^2 + c^2 + d^2"
prerequisites = ["sum-of-two-squares"]
lean4_status = "complete"
+++

## Statement

**Lagrange's four-square theorem** (1770): every non-negative integer $n$ can be expressed as the sum of four perfect squares:

$$n = a^2 + b^2 + c^2 + d^2, \qquad a, b, c, d \in \mathbb{N}.$$

Note that $0^2 = 0$ is allowed, so the four squares need not be positive. By contrast, not every integer is a sum of three squares (see [[Legendre's Three-Square Theorem]]).

## Visualization

Representations of $n = 1$ through $10$ as sums of four squares (minimal number of non-zero terms where possible):

| $n$  | Representation                        |
|------|---------------------------------------|
| 1    | $1^2 + 0^2 + 0^2 + 0^2$              |
| 2    | $1^2 + 1^2 + 0^2 + 0^2$              |
| 3    | $1^2 + 1^2 + 1^2 + 0^2$              |
| 4    | $2^2 + 0^2 + 0^2 + 0^2$              |
| 5    | $2^2 + 1^2 + 0^2 + 0^2$              |
| 6    | $2^2 + 1^2 + 1^2 + 0^2$              |
| 7    | $2^2 + 1^2 + 1^2 + 1^2$              |
| 8    | $2^2 + 2^2 + 0^2 + 0^2$              |
| 9    | $3^2 + 0^2 + 0^2 + 0^2$              |
| 10   | $3^2 + 1^2 + 0^2 + 0^2$              |

The number $7$ is the smallest positive integer that requires all four squares to be non-zero.

## Proof Sketch

1. **Euler's four-square identity.** The product of two sums of four squares is again a sum of four squares:
   $$(a^2+b^2+c^2+d^2)(e^2+f^2+g^2+h^2) = (\text{Hurwitz quaternion norm product}).$$
   This reduces the theorem to prime numbers.
2. **Every prime is a sum of four squares.** For $p = 2$: $2 = 1+1+0+0$. For an odd prime $p$: show that $-1$ is a sum of two squares mod $p$, then apply the pigeonhole principle to find $a^2 + b^2 + 1 \equiv 0 \pmod{p}$ with $|a|, |b| < p/2$. This gives $a^2 + b^2 + 1^2 + 0^2 = kp$ for some $1 \le k < p$. A descent argument reduces $k$ to $1$.
3. **Every integer is a product of primes** (by the [[Fundamental Theorem of Arithmetic]]), so Euler's identity propagates the four-square property.

## Connections

This theorem generalises [[Sum of Two Squares]], which characterises exactly which integers are sums of two squares. The proof technique via quaternion norms is a precursor to the Cayley–Dickson construction, also appearing in [[Cauchy–Schwarz Inequality]] proofs via inner products.

## Lean4 Proof

```lean4
import Mathlib.NumberTheory.SumFourSquares

/-- Lagrange's four-square theorem: every natural number is a sum of four squares.
    Direct alias of `Nat.sum_four_squares` in Mathlib. -/
theorem lagrange (n : ℕ) : ∃ a b c d : ℕ, a ^ 2 + b ^ 2 + c ^ 2 + d ^ 2 = n :=
  Nat.sum_four_squares n

/-- Explicit witness: 7 = 2² + 1² + 1² + 1². -/
theorem four_sq_7 : ∃ a b c d : ℕ, a ^ 2 + b ^ 2 + c ^ 2 + d ^ 2 = 7 :=
  ⟨2, 1, 1, 1, by norm_num⟩
```
