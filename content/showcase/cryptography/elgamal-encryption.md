+++
title = "ElGamal Encryption"
description = "Decryption recovers plaintext because c2/c1^a = m·g^(ab)/g^(ab) = m in any commutative group."
weight = 30
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "c_2 / c_1^a = m"
prerequisites = ["diffie-hellman"]
lean4_status = "complete"
+++

## Statement

Let $G = (\mathbb{Z}/p\mathbb{Z})^*$ with generator $g$. Alice's key pair: private $a$, public $y = g^a$. To encrypt message $m \in G$, Bob picks random $r$ and computes:

$$c_1 = g^r, \quad c_2 = m \cdot y^r = m \cdot g^{ar}$$

Decryption: Alice computes:

$$c_2 / c_1^a = m \cdot g^{ar} / g^{ra} = m$$

The ciphertext $(c_1, c_2)$ hides $m$ under the DH shared secret $g^{ar}$.

## Visualization

Parameters: $g = 5$, $p = 23$, $a = 6$ (Alice's private key), $y = g^a = 8$ (Alice's public key). Encrypt $m = 10$ with Bob's random $r = 3$.

| Computation | Value |
|-------------|-------|
| $c_1 = g^r = 5^3 \bmod 23$ | $c_1 = 125 \bmod 23 = 10$ |
| $y^r = 8^3 \bmod 23$ | $y^r = 512 \bmod 23 = 6$ |
| $c_2 = m \cdot y^r = 10 \cdot 6 \bmod 23$ | $c_2 = 60 \bmod 23 = 14$ |

Decryption:

| Computation | Value |
|-------------|-------|
| $c_1^a = 10^6 \bmod 23$ | $c_1^a = 10^6 \bmod 23 = 6$ |
| $c_1^a$ inverse in $\mathbb{Z}/23\mathbb{Z}$ | $6^{-1} \bmod 23 = 4$ |
| $m' = c_2 \cdot (c_1^a)^{-1} = 14 \cdot 4 \bmod 23$ | $m' = 56 \bmod 23 = 10$ |

Recovered $m' = 10 = m$. Correctness holds: $c_1^a = g^{ra} = g^{ar} = y^r$, so the shared secret cancels.

## Proof Sketch

1. **Expand $c_1^a$.** Since $c_1 = g^r$, we have $c_1^a = g^{ra}$.

2. **Use commutativity.** In an abelian group $g^{ra} = g^{ar} = (g^a)^r = y^r$.

3. **Cancellation.** $c_2 / c_1^a = (m \cdot y^r) / y^r = m$. In group notation, $c_2 \cdot (c_1^a)^{-1} = m \cdot y^r \cdot (y^r)^{-1} = m$.

4. **Security.** Without $a$, an eavesdropper sees $c_1 = g^r$ and $c_2 = m \cdot g^{ar}$; recovering $m$ requires the DH shared secret $g^{ar}$, which is computationally hard.

## Connections

ElGamal is a direct application of [[Diffie–Hellman]] — the shared secret $g^{ar}$ is precisely the DH shared key. The group law relies on [[Fermat's Little Theorem]] for the order of elements in $(\mathbb{Z}/p\mathbb{Z})^*$. The [[Chinese Remainder Theorem]] underlies efficient multi-prime implementations.

## Lean4 Proof

```lean4
/-- ElGamal decryption identity in a commutative group:
    (m * y^r) * (g^r)^(-a_inv) = m  when  y = g^a  and
    we demonstrate the cancellation algebraically. -/

-- Core algebraic fact: in a CommGroup, (g^a)^r = (g^r)^a
theorem elgamal_shared_secret_eq {G : Type*} [CommGroup G] (g : G) (a r : ℕ) :
    (g ^ a) ^ r = (g ^ r) ^ a := by
  rw [← pow_mul, ← pow_mul, Nat.mul_comm]

/-- Cancellation: m * k * k⁻¹ = m in any group. -/
theorem elgamal_cancel {G : Type*} [Group G] (m k : G) :
    m * k * k⁻¹ = m := by
  rw [mul_assoc, mul_inv_cancel, mul_one]

/-- Concrete numeric check: decrypt(encrypt(10)) = 10.
    c1 = 5^3 mod 23 = 125 mod 23 = 10
    y^r = 8^3 mod 23 = 512 mod 23 = 6
    c2 = 10 * 6 mod 23 = 60 mod 23 = 14
    c1^a = 10^6 mod 23 = 6   (= g^(r*a) = g^(a*r) = y^r)
    inv(6) mod 23 = 4  (since 6*4=24≡1 mod 23)
    m' = c2 * inv(c1^a) = 14 * 4 mod 23 = 56 mod 23 = 10  -/
example : 5 ^ 3 % 23 = 10 := by decide
example : (8 : ℕ) ^ 3 % 23 = 6 := by decide
example : 10 * 6 % 23 = 14 := by decide
example : (10 : ℕ) ^ 6 % 23 = 6 := by decide   -- c1^a = y^r confirmed
example : 56 % 23 = 10 := by decide              -- 14 * 4 = 56, recover m
```
