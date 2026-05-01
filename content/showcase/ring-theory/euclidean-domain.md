+++
title = "Euclidean Domain"
description = "A domain with a division algorithm: any a, b give a = qb + r with r smaller than b under a norm function."
weight = 124
tags = ["lean4-proof", "ring-theory", "visualization"]
latex = "\\forall a, b \\ne 0,\\; \\exists q, r:\\; a = qb + r,\\; N(r) < N(b)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A **Euclidean domain** is an integral domain $R$ equipped with a norm function $N : R \setminus \{0\} \to \mathbb{N}$ such that for every $a \in R$ and $b \in R \setminus \{0\}$, there exist $q, r \in R$ with

$$a = qb + r \quad \text{and either } r = 0 \text{ or } N(r) < N(b).$$

**Examples:**
- $\mathbb{Z}$ with $N(n) = |n|$: ordinary integer division.
- $k[x]$ for a field $k$ with $N(f) = \deg f$: polynomial long division.
- $\mathbb{Z}[i]$ (Gaussian integers) with $N(a+bi) = a^2 + b^2$: Euclidean for Gaussian integers.

**Key fact:** Every Euclidean domain is a PID (principal ideal domain).

## Visualization

Division in $\mathbb{Z}[i]$ with $N(a+bi) = a^2 + b^2$: divide $a = 5 + 3i$ by $b = 2 + i$.

Step 1: Compute $a/b$ in $\mathbb{C}$:
$$\frac{5+3i}{2+i} = \frac{(5+3i)(2-i)}{(2+i)(2-i)} = \frac{10-5i+6i-3i^2}{5} = \frac{13+i}{5} = 2.6 + 0.2i$$

Step 2: Round to nearest Gaussian integer: $q = 3 + 0i = 3$.

Step 3: Compute remainder: $r = a - qb = (5+3i) - 3(2+i) = (5+3i) - (6+3i) = -1+0i = -1$.

Step 4: Check $N(r) < N(b)$: $N(-1) = 1 < 5 = N(2+i)$. Division algorithm holds.

| Step | Value |
|------|-------|
| $a$ | $5 + 3i$ |
| $b$ | $2 + i$, $N(b) = 5$ |
| $a/b$ in $\mathbb{C}$ | $2.6 + 0.2i$ |
| $q$ (rounded) | $3$ |
| $r = a - qb$ | $-1$, $N(r) = 1$ |
| Check $N(r) < N(b)$? | $1 < 5$ ✓ |

## Proof Sketch

**Every Euclidean domain is a PID:**

1. Let $I \subseteq R$ be a non-zero ideal. Pick $b \in I \setminus \{0\}$ with $N(b)$ minimal.

2. For any $a \in I$, write $a = qb + r$ with $r = 0$ or $N(r) < N(b)$.

3. Since $a, b \in I$ and $I$ is an ideal, $r = a - qb \in I$.

4. By minimality of $N(b)$, if $r \ne 0$ then $N(r) \ge N(b)$, contradiction. So $r = 0$ and $a = qb$.

5. Therefore $I = (b)$ is principal. Every ideal is principal, so $R$ is a PID.

## Connections

The Euclidean domain property powers the [[Euclidean Algorithm]] for computing GCDs. The chain Euclidean $\to$ PID $\to$ UFD (see [[PID Implies UFD]]) explains why $\mathbb{Z}$ has unique prime factorisation. The same norm argument drives polynomial division in [[Polynomial Division Algorithm]].

## Lean4 Proof

```lean4
-- Mathlib: Int.euclideanDomain
-- in Mathlib.Algebra.EuclideanDomain.Int (line 20)
-- Mathlib: EuclideanDomain.to_principal_ideal_domain
-- in Mathlib.RingTheory.PrincipalIdealDomain (line 265)

/-- ℤ is a Euclidean domain with norm N(n) = |n|. -/
#check (Int.euclideanDomain : EuclideanDomain ℤ)

/-- Every Euclidean domain is a PID. -/
example (R : Type*) [EuclideanDomain R] : IsPrincipalIdealRing R :=
  EuclideanDomain.to_principal_ideal_domain
```
