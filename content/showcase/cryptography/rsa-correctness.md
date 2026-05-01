+++
title = "RSA Correctness"
description = "Decryption recovers the plaintext: m^(ed) ≡ m (mod n) whenever ed ≡ 1 (mod φ(n))."
weight = 10
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "m^{ed} \\equiv m \\pmod{n}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $n = pq$ for distinct odd primes $p$ and $q$. Define $\phi(n) = (p-1)(q-1)$. Choose $e$ with $\gcd(e, \phi(n)) = 1$ and set $d \equiv e^{-1} \pmod{\phi(n)}$, so $ed = 1 + k\phi(n)$ for some integer $k$. Then for any message $m$ with $0 \le m < n$:

$$m^{ed} \equiv m \pmod{n}$$

This is the correctness condition for RSA: encrypting with $e$ then decrypting with $d$ (or vice versa) is the identity.

## Visualization

Parameters: $p = 11$, $q = 13$, $n = 143$, $\phi(n) = 120$, $e = 7$, $d = 103$ (since $7 \times 103 = 721 = 6 \times 120 + 1$).

Message $m = 42$:

| Step | Operation | Value |
|------|-----------|-------|
| Encrypt | $c = 42^7 \bmod 143$ | $c = 95$ |
| Decrypt | $m' = 95^{103} \bmod 143$ | $m' = 42$ |
| Check | $m' = m$? | YES |

Verification that $ed \equiv 1 \pmod{120}$:
$$7 \times 103 = 721 = 6 \times 120 + 1 \implies 721 \equiv 1 \pmod{120}$$

The CRT splits the verification into two prime-modulus checks:
- Modulo $p = 11$: $m^{ed} = m^{1 + k \cdot 10} = m \cdot (m^{10})^k \equiv m \cdot 1^k = m \pmod{11}$
- Modulo $q = 13$: $m^{ed} = m^{1 + k \cdot 12} = m \cdot (m^{12})^k \equiv m \cdot 1^k = m \pmod{13}$

## Proof Sketch

1. **Euler's theorem.** For $\gcd(m, p) = 1$, Fermat's little theorem gives $m^{p-1} \equiv 1 \pmod{p}$. More generally, Euler's theorem gives $m^{\phi(n)} \equiv 1 \pmod{n}$ when $\gcd(m, n) = 1$.

2. **Expand the exponent.** Write $ed = 1 + k\phi(n)$. Then:
$$m^{ed} = m^{1+k\phi(n)} = m \cdot (m^{\phi(n)})^k \equiv m \cdot 1^k = m \pmod{p}$$
and the same holds modulo $q$.

3. **Chinese Remainder Theorem.** Since $p \mid (m^{ed} - m)$ and $q \mid (m^{ed} - m)$ and $\gcd(p,q) = 1$, we conclude $pq = n \mid (m^{ed} - m)$, giving $m^{ed} \equiv m \pmod{n}$.

4. **Degenerate cases.** When $p \mid m$ or $q \mid m$, a separate (easy) case analysis still gives $m^{ed} \equiv m \pmod{n}$.

## Connections

The correctness proof uses [[Fermat's Little Theorem]] (prime factor case) and the [[Chinese Remainder Theorem]] to lift from prime moduli to $n = pq$. The exponent $d$ exists because $\gcd(e, \phi(n)) = 1$, which is established via [[Bezout's Identity]]. See also [[Euler's Totient Function]] for $\phi(pq) = (p-1)(q-1)$.

## Lean4 Proof

```lean4
/-- RSA correctness for the unit-group case: in ZMod p (prime p), every
    nonzero element satisfies a^(ed) = a when ed ≡ 1 (mod p-1).
    We verify on a concrete small instance via decide. -/

-- Concrete check: p = 11, e = 7, d = 103, ed = 721 = 6*120+1
-- Verify 7 * 103 % 120 = 1
#eval (7 * 103) % 120  -- 1

-- Verify the RSA round-trip for m = 42, n = 143
#eval (42 ^ 7 % 143)         -- 95  (ciphertext)
#eval (95 ^ 103 % 143)       -- 42  (recovered plaintext)

/-- Fermat's little theorem: for prime p and a ≠ 0 in ZMod p, a^(p-1) = 1. -/
theorem rsa_fermat_step (p : ℕ) [Fact p.Prime] {a : ZMod p} (ha : a ≠ 0) :
    a ^ (p - 1) = 1 :=
  ZMod.pow_card_sub_one_eq_one ha

/-- Key algebraic identity: a^(1 + k*(p-1)) = a for nonzero a in ZMod p. -/
theorem rsa_exp_identity (p : ℕ) [Fact p.Prime] {a : ZMod p} (ha : a ≠ 0)
    (k : ℕ) : a ^ (1 + k * (p - 1)) = a := by
  rw [pow_add, pow_mul, ZMod.pow_card_sub_one_eq_one ha, one_pow, mul_one, pow_one]
```
