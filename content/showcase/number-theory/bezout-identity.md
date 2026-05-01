+++
title = "Bezout's Identity"
description = "The gcd of two integers is always an integer linear combination of those integers"
weight = 60
tags = ["lean4-proof", "number-theory", "linear-combinations"]
latex = "\\gcd(a,b) = a \\cdot x + b \\cdot y"
prerequisites = ["euclidean-algorithm"]
lean4_status = "complete"
+++

## Statement

For any integers $a$ and $b$, there exist integers $x$ and $y$ such that:

$$\gcd(a, b) = a \cdot x + b \cdot y$$

These are called **Bezout coefficients**. The gcd is the *smallest positive* integer expressible as such a combination, and every integer linear combination of $a$ and $b$ is a multiple of $\gcd(a,b)$.

In particular, $a$ and $b$ are coprime (i.e. $\gcd(a,b)=1$) if and only if there exist integers with $ax + by = 1$.

## Visualization

**Extended Euclidean algorithm** for $\gcd(35, 15)$:

| Step | Equation                              | $x$  | $y$  |
|-----:|:--------------------------------------|-----:|-----:|
|    1 | $35 = 2 \times 15 + 5$               |      |      |
|    2 | $15 = 3 \times 5 + 0$                |      |      |
| Back-sub: $5 = 35 - 2 \times 15$     |  $1$ | $-2$ |

Check: $35 \cdot 1 + 15 \cdot (-2) = 35 - 30 = 5 = \gcd(35, 15)$. ✓

**Lattice of multiples** for $a=6, b=9$:

```
Multiples of 6:  ..., -12, -6,  0,  6, 12, 18, 24, 30, ...
Multiples of 9:  ..., -18, -9,  0,  9, 18, 27, 36, ...
Combinations:    ...,  -3,  0,  3,  6,  9, 12, 15, ...
                       ↑ smallest positive = gcd(6,9) = 3
```

Coefficients: $3 = 6 \cdot (-1) + 9 \cdot 1$ (and also $3 = 6 \cdot 2 + 9 \cdot (-1)$, etc.)

## Proof Sketch

1. **Existence via the algorithm.** The [[Euclidean Algorithm]] maintains coefficients $(s, t)$ such that the current remainder equals $as + bt$. Initialising with $(s,t) = (1,0)$ for $a$ and $(0,1)$ for $b$, each step updates $(s,t)$ by integer row operations. The final nonzero remainder is $\gcd(a,b)$.

2. **Minimality.** Let $d$ be the smallest positive integer linear combination of $a$ and $b$. Then $d \mid a$ and $d \mid b$ (divide with remainder and use minimality), so $d \mid \gcd(a,b)$. But $\gcd(a,b)$ is a linear combination, so $\gcd(a,b) \le d$. Thus $d = \gcd(a,b)$.

3. **All combinations are multiples.** If $c = ax + by$ then $\gcd(a,b) \mid a$ and $\gcd(a,b) \mid b$ implies $\gcd(a,b) \mid c$.

## Connections

Bezout's Identity gives the bridge from the [[Euclidean Algorithm]] to modular arithmetic. It directly implies that in $\mathbb{Z}/p\mathbb{Z}$ every nonzero element has a multiplicative inverse (feeding into [[Fermat's Little Theorem]]). It is also the key lemma in proving the [[Chinese Remainder Theorem]] (constructing the simultaneous solution) and [[Euler's Totient Function]] (characterising units).

## Lean4 Proof

```lean4
/-- **Bezout's identity**: the gcd is an integer linear combination of the two inputs.
    Mathlib provides the Bezout coefficients `Int.gcdA` and `Int.gcdB` and
    proves the identity as `Int.gcd_eq_gcd_ab`. -/
theorem bezout (a b : ℕ) :
    ∃ x y : ℤ, (Nat.gcd a b : ℤ) = a * x + b * y :=
  ⟨Int.gcdA a b, Int.gcdB a b, (Int.gcd_eq_gcd_ab a b).symm⟩
```
