+++
title = "Infinitude of Primes"
description = "Euclid's classic proof that there are infinitely many prime numbers"
weight = 10
difficulty = "introductory"
tags = ["lean4-proof", "number-theory"]
latex = "\\forall\\, n \\in \\mathbb{N},\\; \\exists\\, p > n \\;\\text{such that}\\; p \\;\\text{is prime}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For every natural number $n$, there exists a prime number $p$ greater than $n$. In other words, there is no largest prime. This is a foundational result used in the proof of the [[Fundamental Theorem of Arithmetic]].

$$\forall\, n \in \mathbb{N},\; \exists\, p > n \;\text{such that}\; p \;\text{is prime}$$

## Proof Sketch (Euclid)

1. **Assume** we have some natural number $n$.
2. **Consider** $N = n! + 1$, the factorial of $n$ plus one.
3. **Observe** that $N \geq 2$, so $N$ has at least one prime factor $p$.
4. **Key insight:** Every integer from $1$ to $n$ divides $n!$, so if $p \leq n$ then $p \mid n!$. But $p \mid (n! + 1)$, which would force $p \mid 1$ — a contradiction since $p \geq 2$.
5. **Conclude** that $p > n$.

## Definitions

**IsPrime.** We define $p$ to be prime when:
$$\text{IsPrime}(p) \iff p \geq 2 \;\wedge\; \forall m \in \mathbb{N},\; m \mid p \implies m = 1 \lor m = p$$

**Factorial positivity.** For all $n$:
$$n! \geq 1$$

**Divisibility of factorial.** If $1 \leq p \leq n$, then:
$$p \mid n!$$

## Key Lemma

**Every integer $\geq 2$ has a prime factor:**
$$\forall n \geq 2,\; \exists\, p,\; \text{IsPrime}(p) \;\wedge\; p \mid n$$

This is the essential tool — once we know $n! + 1 \geq 2$, we extract a prime factor and show it must exceed $n$.

## Lean4 Proof

```lean4
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Nat.Factorial.Basic

/-!
# Infinitude of Primes (Euclid's Proof)

A demonstration proof formalized in Lean 4 with Mathlib.
We use Euclid's classical argument: for any n, the number n! + 1
has a prime factor p, and p must be greater than n.
-/

/-- Our custom definition of primality, shown equivalent to Mathlib's `Nat.Prime`. -/
def IsPrime (p : Nat) : Prop :=
  p ≥ 2 ∧ ∀ m : Nat, m ∣ p → m = 1 ∨ m = p

/-- `Nat.Prime p` implies our custom `IsPrime p`. -/
theorem natPrime_to_isPrime {p : Nat} (hp : Nat.Prime p) : IsPrime p :=
  ⟨hp.two_le, hp.eq_one_or_self_of_dvd⟩

/-- Our custom `IsPrime p` implies `Nat.Prime p`. -/
theorem isPrime_to_natPrime {p : Nat} (hp : IsPrime p) : Nat.Prime p :=
  Nat.prime_def.mpr hp

/-- For any n ≥ 2, there exists a prime factor. -/
theorem exists_prime_factor (n : Nat) (hn : n ≥ 2) :
    ∃ p, IsPrime p ∧ p ∣ n := by
  have h : n ≠ 1 := by omega
  obtain ⟨p, hp, hdvd⟩ := Nat.exists_prime_and_dvd h
  exact ⟨p, natPrime_to_isPrime hp, hdvd⟩

/-- n! ≥ 1 for all n. -/
theorem factorial_pos (n : Nat) : Nat.factorial n ≥ 1 :=
  Nat.factorial_pos n

/-- If 1 ≤ p ≤ n then p ∣ n!. -/
theorem dvd_factorial (p n : Nat) (h1 : 1 ≤ p) (h2 : p ≤ n) :
    p ∣ Nat.factorial n :=
  Nat.dvd_factorial (by omega : 0 < p) h2

/-- For every natural number n, there exists a prime greater than n. -/
theorem InfinitudeOfPrimes (n : Nat) :
    ∃ p, p > n ∧ IsPrime p := by
  have h1 : Nat.factorial n + 1 ≥ 2 := by
    have := Nat.factorial_pos n; omega
  obtain ⟨p, hp, hdvd⟩ := exists_prime_factor _ h1
  use p
  constructor
  · by_contra h
    push_neg at h
    have hp2 := hp.1
    have hdvd_fact : p ∣ Nat.factorial n :=
      dvd_factorial p n (by omega) h
    have hdvd_sub := Nat.dvd_sub hdvd hdvd_fact
    rw [Nat.add_sub_cancel_left] at hdvd_sub
    exact absurd (Nat.le_of_dvd Nat.one_pos hdvd_sub) (by omega)
  · exact hp
```
