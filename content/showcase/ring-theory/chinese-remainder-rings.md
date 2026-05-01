+++
title = "CRT for Rings"
description = "Coprime ideals split a quotient ring into a direct product: R/(I₁∩⋯∩Iₙ) ≅ R/I₁ × ⋯ × R/Iₙ."
weight = 127
tags = ["lean4-proof", "ring-theory", "visualization"]
latex = "R/(I_1 \\cap \\cdots \\cap I_n) \\cong R/I_1 \\times \\cdots \\times R/I_n"
prerequisites = ["euclidean-domain"]
lean4_status = "complete"
+++

## Statement

Let $R$ be a commutative ring and $I_1, \ldots, I_n \subseteq R$ ideals that are **pairwise coprime**: $I_j + I_k = R$ for all $j \ne k$.

**Chinese Remainder Theorem (ring version).** There is a ring isomorphism

$$R \big/ \bigcap_{k=1}^n I_k \;\cong\; \prod_{k=1}^n R/I_k.$$

The classical case: $R = \mathbb{Z}$, $I_k = (m_k)$ with $\gcd(m_j, m_k) = 1$ for $j \ne k$, gives $\mathbb{Z}/(m_1 \cdots m_n) \cong \mathbb{Z}/m_1 \times \cdots \times \mathbb{Z}/m_n$.

## Visualization

**Example: $\mathbb{Z}/15 \cong \mathbb{Z}/3 \times \mathbb{Z}/5$.**

Ideals $(3)$ and $(5)$ in $\mathbb{Z}$ are coprime: $3 \cdot 2 + 5 \cdot (-1) = 1$.

| $n \bmod 15$ | $n \bmod 3$ | $n \bmod 5$ | element of $\mathbb{Z}/3 \times \mathbb{Z}/5$ |
|:---:|:---:|:---:|:---:|
| $0$ | $0$ | $0$ | $(0, 0)$ |
| $1$ | $1$ | $1$ | $(1, 1)$ |
| $5$ | $2$ | $0$ | $(2, 0)$ |
| $6$ | $0$ | $1$ | $(0, 1)$ |
| $10$ | $1$ | $0$ | $(1, 0)$ |
| $11$ | $2$ | $1$ | $(2, 1)$ |
| $14$ | $2$ | $4$ | $(2, 4)$ |

**Reconstruction (CRT lift).** Given $(2, 4) \in \mathbb{Z}/3 \times \mathbb{Z}/5$, find $n \bmod 15$.

Step 1: Need $n \equiv 2 \pmod{3}$ and $n \equiv 4 \pmod{5}$.

Step 2: Bezout: $2 \cdot 3 + (-1) \cdot 5 = 1$. So $e_1 = 10 \equiv 1 \pmod{3}$, $e_1 \equiv 0 \pmod{5}$ and $e_2 = 6 \equiv 0 \pmod{3}$, $e_2 \equiv 1 \pmod{5}$.

Step 3: $n = 2 \cdot e_1 + 4 \cdot e_2 = 2 \cdot 10 + 4 \cdot 6 = 20 + 24 = 44 \equiv 14 \pmod{15}$.

## Proof Sketch

1. **Define the map.** Let $\phi : R \to \prod_k R/I_k$ by $\phi(r) = (r + I_1, \ldots, r + I_n)$. This is a ring homomorphism.

2. **Surjectivity.** Given $(a_1, \ldots, a_n) \in \prod R/I_k$, use pairwise coprimality to find idempotents $e_k \in R$ with $e_k \equiv 1 \pmod{I_k}$ and $e_k \equiv 0 \pmod{I_j}$ for $j \ne k$ (via Bezout in $R$). Then $\sum a_k e_k$ maps to $(a_1, \ldots, a_n)$.

3. **Kernel is $\bigcap_k I_k$.** An element $r$ maps to $0$ iff $r \in I_k$ for all $k$, i.e., $r \in \bigcap_k I_k$. By pairwise coprimality, $\bigcap I_k = I_1 \cdots I_n$ (the product ideal).

4. **First Isomorphism Theorem** gives $R/\bigcap I_k \cong \prod R/I_k$.

## Connections

The number-theoretic [[Chinese Remainder Theorem]] is the special case $R = \mathbb{Z}$. The ring version generalises it to arbitrary commutative rings and gives [[Fundamental Theorem of Arithmetic]] a structural home: $\mathbb{Z}/n \cong \prod \mathbb{Z}/p_i^{e_i}$ when $n = \prod p_i^{e_i}$.

## Lean4 Proof

```lean4
-- Mathlib: Ideal.quotientInfRingEquivPiQuotient
-- in Mathlib.RingTheory.Ideal.Quotient.Operations

open Ideal in
/-- CRT for rings: if ideals are pairwise coprime, the quotient by their
    intersection is isomorphic to the product of the individual quotients. -/
#check @Ideal.quotientInfRingEquivPiQuotient
-- quotientInfRingEquivPiQuotient :
--   ∀ {R : Type u_1} [inst : CommRing R] {ι : Type u_2} (f : ι → Ideal R),
--     (∀ i j, i ≠ j → f i ⊔ f j = ⊤) →
--     R ⧸ ⨅ i, f i ≃+* ∀ i, R ⧸ f i

/-- Concrete instance: ℤ/15 ≅ ℤ/3 × ℤ/5. -/
example : ZMod 15 ≃+* ZMod 3 × ZMod 5 :=
  (ZMod.chineseRemainder (by norm_num)).toRingEquiv
```
