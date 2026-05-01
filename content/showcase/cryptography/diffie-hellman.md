+++
title = "Diffie–Hellman"
description = "Alice and Bob derive the same shared secret g^(ab) from public keys g^a and g^b in any commutative group."
weight = 20
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "(g^a)^b = (g^b)^a"
prerequisites = ["rsa-correctness"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a commutative group (written multiplicatively) with generator $g$. Alice picks private exponent $a$, publishes $A = g^a$. Bob picks private exponent $b$, publishes $B = g^b$. The shared secret is:

$$(g^a)^b = g^{ab} = (g^b)^a$$

Both parties can compute this value; an eavesdropper seeing only $g^a$ and $g^b$ must solve the **discrete logarithm problem** to find $a$ or $b$.

## Visualization

Parameters: $g = 5$, $p = 23$ (prime), $a = 6$ (Alice), $b = 15$ (Bob).

| Party | Private | Public | Shared secret computation |
|-------|---------|--------|--------------------------|
| Alice | $a = 6$ | $A = 5^6 \bmod 23 = 8$ | $B^a = 8^6 \bmod 23$ — wait, uses $B = g^b$ |
| Bob | $b = 15$ | $B = 5^{15} \bmod 23 = 19$ | $A^b = 8^{15} \bmod 23$ |

Computing the shared secret:
- Alice computes: $B^a = 19^6 \bmod 23 = 2$
- Bob computes: $A^b = 8^{15} \bmod 23 = 2$

Both arrive at shared secret $K = 2$:

```
Public:  g=5, p=23
Alice:   a=6   ->  A = 5^6  mod 23 = 8
Bob:     b=15  ->  B = 5^15 mod 23 = 19
                           |         |
Alice:   K = 19^6  mod 23 = 2
Bob:     K = 8^15  mod 23 = 2  <-- same!
```

## Proof Sketch

1. **Commutativity of exponents.** In any monoid, $a^{mn} = (a^m)^n$ (associativity of repeated multiplication). In a commutative group, $mn = nm$, so $(g^a)^b = g^{ab} = g^{ba} = (g^b)^a$.

2. **One-line proof.** Rewrite via `pow_mul`: $(g^a)^b = g^{a \cdot b} = g^{b \cdot a} = (g^b)^a$.

3. **Security relies on discrete log hardness.** The equality is trivial group theory; the security assumption is that finding $a$ from $g$ and $g^a$ is computationally hard.

## Connections

The commutativity argument is a direct consequence of the group axioms; see [[Cayley's Theorem]] for the embedding perspective. The hardness assumption connects to [[Fermat's Little Theorem]] — working in $(\mathbb{Z}/p\mathbb{Z})^*$ of order $p-1$. ElGamal encryption (see [[ElGamal Encryption]]) builds directly on DH. The [[Chinese Remainder Theorem]] lets one work component-wise in factored-order groups.

## Lean4 Proof

```lean4
/-- Diffie–Hellman commutativity: in any commutative monoid,
    (g^a)^b = (g^b)^a. This is the core algebraic identity. -/
theorem dh_correct {G : Type*} [CommMonoid G] (g : G) (a b : ℕ) :
    (g ^ a) ^ b = (g ^ b) ^ a := by
  rw [← pow_mul, ← pow_mul, Nat.mul_comm]

/-- Concrete verification for g=5, p=23, a=6, b=15. -/
-- Both sides equal 2 mod 23.
#eval (8 ^ 15) % 23   -- 2  (Bob computes A^b)
#eval (19 ^ 6) % 23   -- 2  (Alice computes B^a)

/-- Numeric check: 5^(6*15) mod 23 = 5^(15*6) mod 23. -/
example : 5 ^ (6 * 15) % 23 = 5 ^ (15 * 6) % 23 := by
  rw [Nat.mul_comm]
```
