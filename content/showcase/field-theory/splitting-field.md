+++
title = "Splitting Field"
description = "The smallest field extension over which a given polynomial factors into linear factors"
weight = 127
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "p = c \\prod_{i=1}^n (x - \\alpha_i) \\in \\text{SplittingField}(p)[x]"
prerequisites = ["algebraic-closure"]
lean4_status = "complete"
+++

## Statement

Given a field $F$ and a non-constant polynomial $p \in F[x]$, the **splitting field** of $p$ over $F$ is the smallest extension $K/F$ in which $p$ factors completely:

$$p = c \prod_{i=1}^{n} (x - \alpha_i) \in K[x], \quad c \in F, \; \alpha_i \in K.$$

In Mathlib: `Polynomial.SplittingField p` is a concrete field, and `Polynomial.IsSplittingField F (SplittingField p) p` records the universal property. The key predicate is `Polynomial.Splits (algebraMap F K) p`.

The splitting field is unique up to $F$-isomorphism and has degree $[K:F] \leq n!$ where $n = \deg p$.

## Visualization

The splitting field of $x^4 - 2$ over $\mathbb{Q}$:

Roots: $\alpha = \sqrt[4]{2}$, $i\alpha$, $-\alpha$, $-i\alpha$ where $\alpha = 2^{1/4} \in \mathbb{R}$.

```
Q(cbrt4(2), i)  [degree 8 over Q]
  /          \
Q(cbrt4(2))   Q(i)
  [deg 4]    [deg 2]
       \       /
          Q
```

The four roots lie in $\mathbb{Q}(\sqrt[4]{2}, i)$ but not all in $\mathbb{Q}(\sqrt[4]{2})$ (missing $i$) or $\mathbb{Q}(i)$ (missing $\sqrt[4]{2}$). The Galois group $\text{Gal}(\mathbb{Q}(\sqrt[4]{2},i)/\mathbb{Q}) \cong D_4$.

Simpler example — splitting field of $x^2 - 2$ over $\mathbb{Q}$:

| Root | Field needed |
|---|---|
| $\sqrt{2}$ | $\mathbb{Q}(\sqrt{2})$ |
| $-\sqrt{2}$ | already in $\mathbb{Q}(\sqrt{2})$ |

So the splitting field is $\mathbb{Q}(\sqrt{2})$, degree 2.

## Proof Sketch

1. **Root adjunction.** If $p$ is irreducible over $F$, adjoin one root $\alpha$ to get $F(\alpha) \cong F[x]/(p)$. Over $F(\alpha)$, $p$ factors off $(x - \alpha)$.
2. **Induction on degree.** Factor off each root in turn. After at most $n$ adjunctions the polynomial is completely split. The result is $F(\alpha_1, \ldots, \alpha_n)$.
3. **Minimality.** Any $F$-extension containing all roots of $p$ contains $F(\alpha_1, \ldots, \alpha_n)$, so the splitting field is minimal.
4. **Uniqueness.** Any two splitting fields are isomorphic over $F$ by induction on the number of roots adjoined.

## Connections

Splitting fields are always [[Normal Extension|normal extensions]] — the defining property of normality is exactly that every irreducible polynomial with one root in $K$ splits completely in $K$. The splitting field construction also appears in the [[Fundamental Theorem of Galois Theory]].

## Lean4 Proof

```lean4
/-- The canonical splitting field of `p` over `F` satisfies the splitting predicate. -/
theorem splittingField_splits (F : Type*) [Field F]
    (p : Polynomial F) :
    p.Splits (algebraMap F (p.SplittingField)) :=
  Polynomial.SplittingField.splits p
```
