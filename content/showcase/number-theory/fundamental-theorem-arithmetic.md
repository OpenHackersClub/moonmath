+++
title = "Fundamental Theorem of Arithmetic"
description = "Every integer greater than 1 has a unique prime factorization"
weight = 20
difficulty = "introductory"
tags = ["lean4-proof", "number-theory"]
latex = "n = p_1^{a_1} \\cdot p_2^{a_2} \\cdots p_k^{a_k}"
prerequisites = ["prime-theorem"]
lean4_status = "sorry"
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
/-- Every natural number > 1 has a prime factor. -/
theorem exists_prime_factor (n : Nat) (hn : n > 1) :
    ∃ p, Nat.Prime p ∧ p ∣ n := by
  induction n using Nat.strong_rec_on with
  | _ n ih =>
    by_cases hp : Nat.Prime n
    · exact ⟨n, hp, dvd_refl n⟩
    · sorry -- factor n = a * b, apply ih to smaller factor

/-- Uniqueness of prime factorization (up to ordering). -/
theorem unique_factorization (n : Nat) (hn : n > 1)
    (ps qs : List Nat)
    (hps : ps.prod = n) (hqs : qs.prod = n)
    (hps_prime : ∀ p ∈ ps, Nat.Prime p)
    (hqs_prime : ∀ q ∈ qs, Nat.Prime q) :
    ps ~ qs := by
  sorry -- by Euclid's lemma and induction on list length
```
