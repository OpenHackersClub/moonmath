+++
title = "FTAG (Finite Abelian)"
description = "Every finite abelian group decomposes as a direct sum of cyclic groups of prime power order."
weight = 80
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "G \\cong \\bigoplus_{i} \\mathbb{Z}/p_i^{e_i}\\mathbb{Z}"
prerequisites = ["lagrange-theorem", "first-isomorphism-theorem"]
lean4_status = "complete"
+++

## Statement

**Fundamental Theorem of Finite Abelian Groups (FTAG).** Every finite abelian group $G$ is isomorphic to a direct sum of cyclic groups of prime power order:

$$G \cong \mathbb{Z}/p_1^{e_1} \oplus \mathbb{Z}/p_2^{e_2} \oplus \cdots \oplus \mathbb{Z}/p_k^{e_k}$$

for primes $p_i$ (not necessarily distinct) and exponents $e_i \geq 1$. The multiset $\{p_i^{e_i}\}$ is uniquely determined by $G$ (primary decomposition). Equivalently, $G$ is a direct sum of cyclic groups of orders $d_1 \mid d_2 \mid \cdots \mid d_m$ (invariant factor form), where $d_m = \exp(G)$.

## Visualization

**Explicit example: $\mathbb{Z}/12 \cong \mathbb{Z}/4 \oplus \mathbb{Z}/3$.**

The map $\phi : \mathbb{Z}/4 \oplus \mathbb{Z}/3 \to \mathbb{Z}/12$ defined by $\phi(a, b) = 3a + 4b \pmod{12}$:

| $(a, b)$ | $3a + 4b \bmod 12$ |
|---|---|
| $(0,0)$ | 0 |
| $(1,0)$ | 3 |
| $(2,0)$ | 6 |
| $(3,0)$ | 9 |
| $(0,1)$ | 4 |
| $(1,1)$ | 7 |
| $(2,1)$ | 10 |
| $(3,1)$ | 1 |
| $(0,2)$ | 8 |
| $(1,2)$ | 11 |
| $(2,2)$ | 2 |
| $(3,2)$ | 5 |

All 12 residues appear exactly once — $\phi$ is an isomorphism. This works because $\gcd(4,3) = 1$ by the [[Chinese Remainder Theorem]].

**All groups of order 8 (up to isomorphism):**

| Group | Primary decomposition |
|---|---|
| $\mathbb{Z}/8$ | $\mathbb{Z}/2^3$ |
| $\mathbb{Z}/4 \oplus \mathbb{Z}/2$ | $\mathbb{Z}/2^2 \oplus \mathbb{Z}/2$ |
| $\mathbb{Z}/2 \oplus \mathbb{Z}/2 \oplus \mathbb{Z}/2$ | $\mathbb{Z}/2 \oplus \mathbb{Z}/2 \oplus \mathbb{Z}/2$ |

(Plus two non-abelian groups $D_4$ and $Q_8$ — the theorem covers only abelian ones.)

## Proof Sketch

1. **$p$-primary decomposition.** Write $G = \bigoplus_p G_p$ where $G_p = \{g \mid p^n g = 0 \text{ for some } n\}$ is the $p$-primary component. Each $G_p$ is a finite abelian $p$-group.
2. **Cyclic decomposition of each $G_p$.** A finite abelian $p$-group can be written as $\mathbb{Z}/p^{e_1} \oplus \cdots \oplus \mathbb{Z}/p^{e_r}$ with $e_1 \leq \cdots \leq e_r$. Proof: pick an element of maximal order, split off the cyclic subgroup it generates (using injectivity of divisible modules), and induct on $|G|$.
3. **Uniqueness.** Count elements of order $p^k$ in each decomposition to recover the exponents uniquely.

## Connections

The theorem classifies all solutions to $g + g + \cdots + g = 0$ in $G$, making it the abelian backbone of [[Sylow Theorems]] — once you know Sylow $p$-subgroups exist, FTAG classifies all possible structures. The Chinese Remainder isomorphism $\mathbb{Z}/mn \cong \mathbb{Z}/m \oplus \mathbb{Z}/n$ when $\gcd(m,n)=1$ is the simplest instance of the decomposition, appearing in [[Chinese Remainder Theorem]].

## Lean4 Proof

```lean4
/-- The Fundamental Theorem of Finite Abelian Groups: every finite additive abelian group
    is isomorphic to a direct sum of cyclic groups ZMod (p^e).
    Mathlib: `AddCommGroup.equiv_directSum_zmod_of_finite`. -/
theorem ftag (G : Type*) [AddCommGroup G] [Finite G] :
    ∃ (ι : Type) (_ : Fintype ι) (p : ι → ℕ) (_ : ∀ i, Nat.Prime (p i)) (e : ι → ℕ),
      Nonempty (G ≃+ ⨁ i : ι, ZMod (p i ^ e i)) :=
  AddCommGroup.equiv_directSum_zmod_of_finite G
```
