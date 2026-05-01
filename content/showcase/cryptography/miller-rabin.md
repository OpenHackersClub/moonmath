+++
title = "Miller–Rabin Primality Test"
description = "A Fermat witness a^(n-1) ≢ 1 (mod n) certifies compositeness; Carmichael numbers fool every base."
weight = 50
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "a^{n-1} \\not\\equiv 1 \\pmod{n} \\implies n \\text{ composite}"
prerequisites = ["modular-exponentiation"]
lean4_status = "complete"
+++

## Statement

**Fermat's compositeness witness.** If $n > 2$ is prime and $\gcd(a, n) = 1$, then Fermat's little theorem gives $a^{n-1} \equiv 1 \pmod{n}$. The contrapositive:

$$a^{n-1} \not\equiv 1 \pmod{n} \implies n \text{ is composite}$$

Such an $a$ is called a **Fermat witness** for the compositeness of $n$.

**Carmichael numbers.** There exist composite $n$ (Carmichael numbers) for which $a^{n-1} \equiv 1 \pmod{n}$ for every $a$ with $\gcd(a, n) = 1$. The smallest is $n = 561 = 3 \times 11 \times 17$. The Miller–Rabin test strengthens Fermat's test to eliminate Carmichael numbers by additionally checking square roots of $1$.

## Visualization

Test $n = 561$ (Carmichael number) with base $a = 2$:

Write $n - 1 = 560 = 16 \times 35 = 2^4 \times 35$. Compute the sequence $2^{35}, 2^{70}, 2^{140}, 2^{280}, 2^{560}$ modulo $561$:

```
2^35   mod 561 = 263
2^70   mod 561 = 166   (263^2 mod 561)
2^140  mod 561 = 67    (166^2 mod 561)
2^280  mod 561 = 1     (67^2 mod 561)
2^560  mod 561 = 1     (1^2 mod 561)
```

The final value is $1$ (Fermat test passes), but $2^{280} \equiv 1$ and $2^{140} \equiv 67 \ne \pm 1 \pmod{561}$. This is a non-trivial square root of $1$, impossible modulo a prime — so Miller–Rabin correctly rejects $561$.

For comparison, test $n = 13$ (prime) with $a = 2$:

| $k$ | $2^k \bmod 13$ |
|-----|----------------|
| $1$ | $2$ |
| $6$ | $64 \bmod 13 = 12 \equiv -1$ |
| $12$ | $1$ |

$2^{12} \equiv 1$ and the sequence hits $-1$ before $1$ — consistent with primality.

Carmichael witness table for $n = 561$:

| Property | Value |
|----------|-------|
| $n - 1 = 2^s \cdot d$ | $s = 4$, $d = 35$ |
| $a = 2$, $a^d \bmod n$ | $263$ |
| Fermat test $a^{n-1} \bmod n$ | $1$ (passes — false!) |
| Miller–Rabin verdict | COMPOSITE (non-trivial sqrt of 1) |

## Proof Sketch

1. **Fermat witness.** If $n$ is prime and $\gcd(a, n) = 1$, Fermat's little theorem says $a^{n-1} \equiv 1 \pmod{n}$. Contrapositive: $a^{n-1} \not\equiv 1$ certifies $n$ is composite.

2. **Square root argument.** In $\mathbb{Z}/p\mathbb{Z}$ for prime $p$, the only solutions to $x^2 \equiv 1$ are $x \equiv \pm 1$. So in the sequence $a^d, a^{2d}, \ldots, a^{2^s d} = a^{n-1}$, the first $1$ must be preceded by $-1$ (or the sequence starts at $1$).

3. **Carmichael numbers fail this.** Since $n$ is composite, $\mathbb{Z}/n\mathbb{Z}$ may have more square roots of $1$. Detecting them certifies compositeness.

4. **Probabilistic guarantee.** For a random composite $n$, at least $3/4$ of bases $a$ are Miller–Rabin witnesses. Running $k$ rounds gives error probability $\le (1/4)^k$.

## Connections

Miller–Rabin rests on [[Fermat's Little Theorem]] (the prime case). The square-root-of-$1$ argument uses that $\mathbb{Z}/p\mathbb{Z}$ is a field; compare with [[Wilson's Theorem]] which characterises primes via $(p-1)! \equiv -1$. Large primes are generated for [[RSA Correctness]] using exactly this test.

## Lean4 Proof

```lean4
/-- Fermat's compositeness witness: if n is prime then a^(n-1) ≡ 1 mod n
    (for gcd(a,n)=1). We state the contrapositive concretely. -/

-- In ZMod p (prime p), every nonzero element satisfies a^(p-1) = 1.
theorem fermat_prime_pow (p : ℕ) [Fact p.Prime] {a : ZMod p} (ha : a ≠ 0) :
    a ^ (p - 1) = 1 :=
  ZMod.pow_card_sub_one_eq_one ha

/-- Concrete check: 561 is composite. -/
example : ¬ Nat.Prime 561 := by decide

/-- 2^560 mod 561 = 1 (Fermat test passes — 561 is a Carmichael number). -/
example : 2 ^ 560 % 561 = 1 := by decide

/-- But 2^140 mod 561 ≠ 1 and ≠ 560 (i.e. ≠ -1 mod 561).
    This is the non-trivial square root of 1 that Miller–Rabin detects. -/
example : 2 ^ 140 % 561 = 67 := by decide
example : (67 : ℕ) ≠ 1 := by decide
example : (67 : ℕ) ≠ 560 := by decide
```
