+++
title = "Fundamental Theorem of Arithmetic"
description = "Every integer greater than 1 has a unique prime factorization"
weight = 20
tags = ["lean4-proof", "number-theory"]
latex = "n = p_1^{a_1} \\cdot p_2^{a_2} \\cdots p_k^{a_k}"
prerequisites = ["prime-theorem"]
lean4_status = "complete"
+++

## Statement

Every integer $n > 1$ can be written as a product of prime numbers:

$$n = p_1^{a_1} \cdot p_2^{a_2} \cdots p_k^{a_k}$$

and this representation is unique up to the order of the factors.

## Proof Outline

### Existence (by strong induction)

- **Base case:** $n = 2$ is prime, so it is its own factorization.
- **Inductive step:** If $n > 2$, either $n$ is prime (done) or $n = ab$ with $1 < a, b < n$. By the inductive hypothesis, both $a$ and $b$ have prime factorizations, so $n$ does too.

### Uniqueness (by contradiction)

Suppose $n = p_1 p_2 \cdots p_j = q_1 q_2 \cdots q_k$ are two prime factorizations. Since $p_1 \mid q_1 q_2 \cdots q_k$ and $p_1$ is prime, Euclid's lemma gives $p_1 \mid q_i$ for some $i$. Since $q_i$ is also prime, $p_1 = q_i$. Cancel and repeat by induction.

## Connections

The existence part relies on the [[Infinitude of Primes]]. This theorem is in turn a prerequisite for [[Fermat's Little Theorem]] and many results in modular arithmetic.

## Lean4 Proof

```lean4
/-- Existence: every nonzero natural number equals the product of its
    prime factors raised to their multiplicities. The
    `Nat.factorization` finsupp records each multiplicity, and Mathlib
    proves the product reconstructs the original number. -/
theorem fta_existence (n : ℕ) (hn : n ≠ 0) :
    n.factorization.prod (fun p k => p ^ k) = n :=
  Nat.factorization_prod_pow_eq_self hn

/-- Uniqueness: two natural numbers with the same prime factorization
    are equal. Since `Nat.factorization` is a function, equal
    factorizations force equal nonzero numbers. -/
theorem fta_uniqueness {m n : ℕ} (hm : m ≠ 0) (hn : n ≠ 0)
    (h : m.factorization = n.factorization) : m = n :=
  Nat.eq_of_factorization_eq hm hn (fun p => by rw [h])
```
