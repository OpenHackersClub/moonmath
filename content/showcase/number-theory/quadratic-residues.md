+++
title = "Quadratic Residues"
description = "An integer a is a quadratic residue mod p when it is a nonzero perfect square in Z/pZ"
weight = 150
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "a \\text{ is a QR mod } p \\iff \\exists x,\\; x^2 \\equiv a \\pmod{p}"
prerequisites = ["fermats-little-theorem", "eulers-totient"]
lean4_status = "complete"
+++

## Statement

Let $p$ be an odd prime. An integer $a$ with $p \nmid a$ is called a **quadratic residue** (QR) mod $p$ if there exists $x$ with $x^2 \equiv a \pmod{p}$; otherwise it is a **quadratic non-residue** (QNR). Exactly $(p-1)/2$ residues are QRs and $(p-1)/2$ are QNRs.

A classical characterisation (Euler's criterion): for $p \nmid a$,

$$a^{(p-1)/2} \equiv \begin{cases} 1 \pmod{p} & a \text{ is a QR mod } p \\ -1 \pmod{p} & a \text{ is a QNR mod } p \end{cases}$$

For $a = -1$ the criterion gives a clean answer: $-1$ is a QR mod $p$ if and only if $p \equiv 1 \pmod{4}$.

## Visualization

**Quadratic residues mod 7** (squares of $1,\ldots,6$ reduced mod 7):

| $x$ | $x^2 \bmod 7$ |
|-----|--------------|
| 1   | 1            |
| 2   | 4            |
| 3   | 2            |
| 4   | 2            |
| 5   | 4            |
| 6   | 1            |

QRs mod 7: $\{1, 2, 4\}$. QNRs mod 7: $\{3, 5, 6\}$.

**Quadratic residues mod 13** (squares of $1,\ldots,12$ reduced mod 13):

| $x$   | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
|--------|---|---|---|---|---|---|---|---|---|----|----|----|
| $x^2 \bmod 13$ | 1 | 4 | 9 | 3 | 12 | 10 | 10 | 12 | 3 | 9 | 4 | 1 |

QRs mod 13: $\{1, 3, 4, 9, 10, 12\}$. QNRs: $\{2, 5, 6, 7, 8, 11\}$.

Note: $p = 13 \equiv 1 \pmod{4}$, so $-1 \equiv 12$ is a QR — confirmed above.

## Proof Sketch

1. The map $x \mapsto x^2$ on $(\mathbb{Z}/p\mathbb{Z})^\times$ is 2-to-1 (since $x^2 = (-x)^2$ and $x \neq -x$ for $p$ odd), so exactly $(p-1)/2$ elements are QRs.
2. Every element $a$ of $(\mathbb{Z}/p\mathbb{Z})^\times$ satisfies $a^{p-1} = 1$ by [[Fermat's Little Theorem]], so $a^{(p-1)/2}$ is a square root of $1$, hence $\pm 1$.
3. The $QRs$ are exactly the kernel of $a \mapsto a^{(p-1)/2}$ (the $(p-1)/2$-th power map), giving Euler's criterion.
4. For $a = -1$: $(-1)^{(p-1)/2} = 1$ iff $(p-1)/2$ is even iff $p \equiv 1 \pmod{4}$.

## Connections

Quadratic residues are the language in which [[Quadratic Reciprocity]] is stated: the Legendre symbol encodes whether $a$ is a QR mod $p$. The count of QRs is governed by [[Euler's Totient Function]] through the index of the subgroup of squares in $(\mathbb{Z}/p\mathbb{Z})^\times$.

## Lean4 Proof

```lean4
/-- -1 is a square in ZMod p iff p % 4 ≠ 3.
    Mathlib's `ZMod.exists_sq_eq_neg_one_iff` states this directly. -/
theorem neg_one_is_qr (p : ℕ) [Fact p.Prime] :
    IsSquare (-1 : ZMod p) ↔ p % 4 ≠ 3 :=
  ZMod.exists_sq_eq_neg_one_iff
```
