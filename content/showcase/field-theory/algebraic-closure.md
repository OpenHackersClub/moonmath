+++
title = "Algebraic Closure"
description = "Every field embeds into an algebraically closed field in which every non-constant polynomial has a root"
weight = 126
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "\\overline{F} = \\{\\alpha \\mid \\alpha \\text{ algebraic over } F\\}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Every field $F$ has an **algebraic closure** $\overline{F}$: an algebraic extension of $F$ that is itself algebraically closed. Algebraically closed means every non-constant polynomial $p \in \overline{F}[x]$ splits completely into linear factors over $\overline{F}$.

In Mathlib this is the type `AlgebraicClosure F`, and the instance `IsAlgClosed (AlgebraicClosure F)` confirms it is algebraically closed.

Two fundamental facts:
1. $\overline{F}$ is algebraic over $F$ — every element satisfies a polynomial with coefficients in $F$.
2. $\overline{F}$ is unique up to (non-canonical) $F$-isomorphism.

## Visualization

The chain $\mathbb{Q} \subset \overline{\mathbb{Q}} \subset \mathbb{C}$ illustrates the layers:

```
C  (transcendentals: e, pi, ...)
|
Q-bar  (algebraic closure of Q)
|  algebraic elements: sqrt(2), i, cbrt(5), sqrt(2)+sqrt(3), ...
|
Q  (base field)
```

Concrete algebraic numbers over $\mathbb{Q}$:

| Element | Minimal polynomial |
|---|---|
| $\sqrt{2}$ | $x^2 - 2$ |
| $i$ | $x^2 + 1$ |
| $\sqrt[3]{5}$ | $x^3 - 5$ |
| $\sqrt{2} + \sqrt{3}$ | $x^4 - 10x^2 + 1$ |
| $\zeta_7 = e^{2\pi i/7}$ | $x^6 + x^5 + x^4 + x^3 + x^2 + x + 1$ |

Every polynomial with coefficients in $\overline{\mathbb{Q}}$ splits completely inside $\overline{\mathbb{Q}}$. In contrast, $\mathbb{Q}$ is not closed ($x^2 - 2$ has no rational root) and $\mathbb{R}$ is not closed ($x^2 + 1$ has no real root).

## Proof Sketch

1. **Existence via Zorn's Lemma.** Form the set of all algebraic extensions of $F$ ordered by inclusion. By Zorn's Lemma (applied carefully to a large enough ambient universe) a maximal element exists; this maximal algebraic extension is algebraically closed.
2. **Algebraically closed check.** If every polynomial over $F$ had a root in $E$ but $E$ were not closed, we could adjoin a root of some polynomial over $E$ to get a strictly larger algebraic extension, contradicting maximality.
3. **Uniqueness.** Any two algebraic closures $\overline{F}_1, \overline{F}_2$ are isomorphic via an $F$-algebra isomorphism; the isomorphism is constructed by transfinite induction, extending partial maps root by root.

## Connections

The algebraic closure is the ambient field for [[Fundamental Theorem of Algebra]] (which shows $\overline{\mathbb{R}} = \mathbb{C}$). It also underpins [[Splitting Field]], since the splitting field of any polynomial lives inside $\overline{F}$.

## Lean4 Proof

```lean4
/-- `AlgebraicClosure F` is algebraically closed.
    Mathlib registers this as an instance. -/
theorem algebraicClosure_isAlgClosed (F : Type*) [Field F] :
    IsAlgClosed (AlgebraicClosure F) :=
  AlgebraicClosure.isAlgClosed F
```
