+++
title = "Wilson's Theorem"
description = "A natural number p > 1 is prime if and only if (p-1)! ≡ -1 mod p"
weight = 80
tags = ["lean4-proof", "number-theory", "primality", "factorials"]
latex = "(p-1)! \\equiv -1 \\pmod{p}"
prerequisites = ["fermats-little-theorem"]
lean4_status = "complete"
+++

## Statement

A natural number $p > 1$ is prime **if and only if**:

$$(p-1)! \equiv -1 \pmod{p}$$

The forward direction says primes make the factorial congruent to $-1$; the reverse says any composite $n$ fails this test (since some prime factor $q < n$ divides both $n$ and $(n-1)!$, so $(n-1)! \not\equiv -1 \pmod{n}$).

## Visualization

**Small cases**:

| $p$ | $(p-1)!$ | $(p-1)! \bmod p$ | Prime? |
|----:|----------:|------------------:|-------:|
|  2  | $1! = 1$  | $1 \equiv -1$    | yes    |
|  3  | $2! = 2$  | $2 \equiv -1$    | yes    |
|  4  | $3! = 6$  | $6 \equiv 2$     | no (6 ≢ -1≡3) |
|  5  | $4! = 24$ | $24 \equiv -1$   | yes    |
|  6  | $5!=120$  | $120 \equiv 0$   | no     |
|  7  | $6!=720$  | $720 \equiv -1$  | yes    |
| 11  | $10!$     | $3628800 \equiv -1$ | yes |

**Why pairing works** (for prime $p = 7$):

In $\mathbb{Z}/7\mathbb{Z}$, every element $1 \le a \le 5$ has a unique inverse $a^{-1} \ne a$:

```
1 · 1 = 1        (self-inverse)
2 · 4 = 8 ≡ 1   (pair)
3 · 5 = 15 ≡ 1  (pair)
6 · 6 = 36 ≡ 1  (self-inverse: 6 ≡ -1)
```

Multiply everything: $6! = 1 \cdot (2 \cdot 4) \cdot (3 \cdot 5) \cdot 6 \equiv 1 \cdot 1 \cdot 1 \cdot (-1) = -1 \pmod 7$. ✓

**Key insight**: the only self-inverse elements mod $p$ (prime) are $\pm 1$, because $a^2 \equiv 1 \implies p \mid (a-1)(a+1) \implies a \equiv \pm 1$.

## Proof Sketch

1. **Pairing.** For prime $p$, each element $a \in \{2, \ldots, p-2\}$ has a unique inverse $a^{-1} \ne a$ (since $a^2 \equiv 1 \pmod p$ has only solutions $a \equiv \pm 1$). These elements pair up.

2. **Product of pairs.** $\prod_{a=2}^{p-2} a \equiv 1 \pmod p$ (each pair contributes a factor of 1).

3. **Boundary terms.** Include $1$ and $p-1 \equiv -1$: the full product $(p-1)! \equiv 1 \cdot 1 \cdot (-1) = -1 \pmod p$.

4. **Composite direction.** If $n = ab$ with $1 < a, b < n$, then $a \mid (n-1)!$ and $a \mid n$, so $a \mid \gcd((n-1)!, n)$. Thus $(n-1)! \not\equiv -1 \pmod n$ (since $-1$ is a unit).

## Connections

Wilson's theorem gives a *perfect* primality characterisation — but it is computationally impractical (computing $(p-1)!$ is exponential in $p$). It connects to [[Fermat's Little Theorem]] (which applies to *all* elements, not just the factorial) and to [[Euler's Totient Function]] (the group $(\mathbb{Z}/p\mathbb{Z})^\times$ has order $p-1$ and product $-1$ for prime $p$). The pairing argument is also central to [[Quadratic Reciprocity]] proofs. See also [[Infinitude of Primes]] for why primes are abundant enough to make this interesting.

## Lean4 Proof

```lean4
/-- **Wilson's theorem**: for a prime `p`, the factorial `(p-1)!` is congruent
    to `-1` in `ZMod p`. Mathlib provides this as `ZMod.wilsons_lemma`. -/
theorem wilson (p : ℕ) [Fact p.Prime] : ((p - 1)! : ZMod p) = -1 :=
  ZMod.wilsons_lemma p
```
