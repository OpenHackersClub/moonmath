+++
title = "Transcendence Basis"
description = "Every field extension has a transcendence basis: a maximal algebraically independent set over the base field"
weight = 132
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "K/F \\text{ algebraic over } F(B) \\text{ and } B \\text{ algebraically independent over } F"
prerequisites = ["algebraic-closure"]
lean4_status = "complete"
+++

## Statement

Let $K/F$ be a field extension. A set $B \subseteq K$ is a **transcendence basis** if:
1. $B$ is **algebraically independent** over $F$: no non-zero polynomial in $F[x_1, \ldots, x_n]$ vanishes on any finite tuple from $B$.
2. $K$ is **algebraic** over $F(B)$: every element of $K$ satisfies a polynomial with coefficients in $F(B)$.

Every field extension has a transcendence basis (by Zorn's Lemma), and all transcendence bases have the same cardinality — the **transcendence degree** $\text{tr.deg}(K/F)$.

In Mathlib: `IsTranscendenceBasis F x` asserts that the indexed family $x : \iota \to K$ is a transcendence basis. Existence is `exists_isTranscendenceBasis`.

## Visualization

**Transcendence degree 0** — $\overline{\mathbb{Q}}/\mathbb{Q}$: the extension is algebraic, so $B = \emptyset$ is a transcendence basis.

**Transcendence degree 1** — $\mathbb{Q}(t)/\mathbb{Q}$ (rational functions): $B = \{t\}$ is transcendental and $\mathbb{Q}(t)$ is trivially algebraic over $\mathbb{Q}(t)$.

**Transcendence degree 2** — $\mathbb{C}/\mathbb{Q}$: $|\mathbb{C}| = 2^{\aleph_0}$ so $\text{tr.deg}(\mathbb{C}/\mathbb{Q}) = 2^{\aleph_0}$.

```
K = C
|  algebraic over Q(B)
F(B) = Q(e, pi, ...)    <-- B is a transcendence basis (uncountable)
|  purely transcendental
F = Q
```

**Open problem.** Are $e$ and $\pi$ algebraically independent over $\mathbb{Q}$? It is known each is transcendental (Hermite 1873, Lindemann 1882), but whether $\{e, \pi\}$ forms an algebraically independent set is unknown. If yes, $\text{tr.deg}(\mathbb{Q}(e,\pi)/\mathbb{Q}) = 2$; if no (e.g., $e = \pi + r$ for some $r \in \overline{\mathbb{Q}}$) it would equal 1.

**Concrete algebraic independence check.** The set $\{x, y\}$ in $\mathbb{Q}(x,y)$ is algebraically independent over $\mathbb{Q}$: if $f(x,y) = 0$ for a polynomial $f$ over $\mathbb{Q}$, then $f = 0$ as a polynomial (clear from the construction of the rational function field).

| Extension | Transcendence basis | $\text{tr.deg}$ |
|---|---|---|
| $\overline{\mathbb{Q}}/\mathbb{Q}$ | $\emptyset$ | $0$ |
| $\mathbb{Q}(t)/\mathbb{Q}$ | $\{t\}$ | $1$ |
| $\mathbb{Q}(s,t)/\mathbb{Q}$ | $\{s,t\}$ | $2$ |
| $\mathbb{C}/\mathbb{Q}$ | uncountable | $2^{\aleph_0}$ |

## Proof Sketch

1. **Existence (Zorn's Lemma).** The set of algebraically independent subsets of $K$ over $F$ is partially ordered by inclusion. Every chain has an upper bound (the union). By Zorn's Lemma, a maximal element $B$ exists. Maximality implies $K$ is algebraic over $F(B)$ (else we could adjoin a transcendental element to $B$).
2. **Cardinality invariance.** If $B$ and $B'$ are both transcendence bases, a standard exchange argument (analogous to linear independence/basis exchange) shows $|B| = |B'|$.
3. **Analogy with linear algebra.** Transcendence bases are to algebraic independence as vector space bases are to linear independence: existence by Zorn, uniqueness of cardinality.

## Connections

Transcendence bases classify field extensions up to the "purely transcendental + algebraic" factorization: $F \subseteq F(B) \subseteq K$ with $F(B)/F$ purely transcendental and $K/F(B)$ algebraic. This decomposition appears in the classification of algebraically closed fields — two algebraically closed fields of the same characteristic are isomorphic iff they have the same transcendence degree (closely related to the [[Algebraic Closure]] uniqueness theorem). The concept also interacts with [[Separable Extension]]: a separably generated extension has a transcendence basis over which the extension is separable.

## Lean4 Proof

```lean4
/-- Every field extension has a transcendence basis (Zorn's Lemma). -/
theorem field_has_transcendence_basis (F K : Type*) [Field F] [Field K]
    [Algebra F K] [FaithfulSMul F K] :
    ∃ s : Set K, IsTranscendenceBasis F ((↑) : s → K) :=
  exists_isTranscendenceBasis F K
```
