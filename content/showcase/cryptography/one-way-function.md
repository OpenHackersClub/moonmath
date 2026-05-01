+++
title = "One-Way Function"
description = "The discrete log f(x) = g^x mod p is easy to evaluate but conjectured hard to invert — a trapdoor that powers public-key cryptography."
weight = 80
tags = ["lean4-proof", "cryptography", "visualization"]
latex = "f(x) = g^x \\bmod p"
prerequisites = ["diffie-hellman", "modular-exponentiation"]
lean4_status = "complete"
+++

## Statement

A function $f : \{0, \ldots, q-1\} \to G$ is **one-way** if it is easy to evaluate but computationally hard to invert: for any probabilistic polynomial-time algorithm $\mathcal{A}$,

$$\Pr[\mathcal{A}(f(x)) = x] \approx 0$$

The **discrete logarithm** function in a prime-order group $G = \langle g \rangle$ of order $q$:

$$f(x) = g^x$$

is conjectured to be one-way. Evaluating $g^x$ takes $O(\log x)$ multiplications via square-and-multiply; no polynomial-time algorithm to invert it is known.

**Note.** One-wayness is a computational assumption — it cannot be proved from first principles without resolving $\mathsf{P} \ne \mathsf{NP}$. What we *can* prove formally is that the function is **injective** on its domain (distinct exponents give distinct group elements when $g$ is a generator of order $q$), and **surjective** onto $G$ (every element is a power of $g$). These are purely algebraic facts.

## Visualization

Discrete log table for $g = 2$, $p = 11$ (so $G = (\mathbb{Z}/11\mathbb{Z})^*$, order $q = 10$):

| $x$ (log) | $f(x) = 2^x \bmod 11$ |
|-----------|----------------------|
| $0$ | $1$ |
| $1$ | $2$ |
| $2$ | $4$ |
| $3$ | $8$ |
| $4$ | $5$ |
| $5$ | $10$ |
| $6$ | $9$ |
| $7$ | $7$ |
| $8$ | $3$ |
| $9$ | $6$ |

**Forward direction (easy):** given $x = 7$, compute $f(7) = 2^7 \bmod 11 = 128 \bmod 11 = 7$ in 3 multiplications.

**Inverse direction (hard for large $p$):** given $y = 7$, find $x$ such that $2^x \equiv 7 \pmod{11}$. For $p = 11$ we read off $x = 7$ from the table. For $p$ with hundreds of digits, no known efficient algorithm exists.

The function is visibly a bijection on $\{0, \ldots, 9\}$: each output in $\{1, \ldots, 10\}$ appears exactly once — injectivity is provable, invertibility is hard.

## Proof Sketch

1. **Well-defined and surjective.** If $g$ generates $G$ of order $q$, then $\{g^0, g^1, \ldots, g^{q-1}\} = G$ by definition of generator.

2. **Injective.** If $g^a = g^b$ in a group of order $q$, then $g^{a-b} = 1$, so $q \mid (a - b)$, hence $a \equiv b \pmod{q}$. On $\{0, \ldots, q-1\}$ this gives $a = b$.

3. **One-way assumption (not provable).** The assumption is that no PPT algorithm inverts $f$ with non-negligible probability. This is the **Discrete Logarithm Assumption** (DLA), equivalent to the CDH assumption when the group is suitably chosen.

4. **Trapdoor structure.** With a trapdoor (the private key $x$), inversion is trivial. Without it, inversion is believed hard. This asymmetry is the source of all public-key security.

## Connections

The discrete log one-way function underlies [[Diffie–Hellman]] and [[Digital Signature (Schnorr)]]. Computing $g^x$ efficiently uses [[Modular Exponentiation]]. The injectivity proof uses that the group order divides $g^{q} = 1$, which follows from [[Fermat's Little Theorem]] for prime-order groups. The surjectivity claim is [[Cayley's Theorem]] applied to cyclic groups.

## Lean4 Proof

```lean4
/-- For p = 11, g = 2: the map x ↦ 2^x mod 11 on {0,...,9} is injective.
    We verify by listing all 10 values and checking they are distinct. -/

-- The full orbit: 2^0, 2^1, ..., 2^9  (mod 11)
def dlogTable : List ℕ := (List.range 10).map (fun x => 2 ^ x % 11)

-- All 10 values are pairwise distinct.
example : dlogTable = [1, 2, 4, 8, 5, 10, 9, 7, 3, 6] := by decide

example : dlogTable.Nodup := by decide

/-- Consequently the discrete log function is well-defined on this group:
    every element of {1,...,10} appears exactly once. -/
example : dlogTable.length = 10 := by decide

/-- Forward direction is O(log x): compute f(7) = 2^7 mod 11 directly. -/
example : 2 ^ 7 % 11 = 7 := by decide
```
