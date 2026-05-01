+++
title = "Normal Extension"
description = "A field extension where every irreducible polynomial with one root in the extension splits completely"
weight = 129
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "K/F \\text{ normal} \\iff \\forall f \\in F[x],\\; f \\text{ irred., has a root in } K \\implies f \\text{ splits in } K"
prerequisites = ["splitting-field", "separable-extension"]
lean4_status = "complete"
+++

## Statement

An algebraic extension $K/F$ is **normal** if every irreducible polynomial $f \in F[x]$ that has at least one root in $K$ splits completely over $K$:

$$K/F \text{ normal} \iff \forall f \in F[x] \text{ irred.},\; f \text{ has a root in } K \implies f \text{ splits in } K.$$

Equivalently, $K/F$ is normal iff $K$ is the splitting field of some (possibly infinite) family of polynomials over $F$.

In Mathlib: `Normal F K` is the predicate. Splitting fields are always normal: `Normal.of_isSplittingField`.

A **Galois extension** is one that is both normal and separable.

## Visualization

**Not normal** — $\mathbb{Q}(\sqrt[3]{2})/\mathbb{Q}$:

The minimal polynomial of $\sqrt[3]{2}$ over $\mathbb{Q}$ is $x^3 - 2$, which has three roots:

$$x^3 - 2 = (x - \sqrt[3]{2})(x - \omega\sqrt[3]{2})(x - \omega^2\sqrt[3]{2}), \quad \omega = e^{2\pi i/3}$$

Only $\sqrt[3]{2}$ lies in $\mathbb{Q}(\sqrt[3]{2}) \subset \mathbb{R}$. The complex roots $\omega\sqrt[3]{2}$ and $\omega^2\sqrt[3]{2}$ are not real, so $x^3 - 2$ does not split in $\mathbb{Q}(\sqrt[3]{2})$.

**Normal** — $\mathbb{Q}(\sqrt{2})/\mathbb{Q}$:

The minimal polynomial $x^2 - 2$ has roots $\pm\sqrt{2}$, both in $\mathbb{Q}(\sqrt{2})$. Extension is the splitting field of $x^2 - 2$.

| Extension | Polynomial | Roots in extension | Normal? |
|---|---|---|---|
| $\mathbb{Q}(\sqrt{2})/\mathbb{Q}$ | $x^2 - 2$ | $\sqrt{2}, -\sqrt{2}$ | Yes |
| $\mathbb{Q}(\sqrt[3]{2})/\mathbb{Q}$ | $x^3 - 2$ | only $\sqrt[3]{2}$ | No |
| $\mathbb{Q}(\sqrt[4]{2}, i)/\mathbb{Q}$ | $x^4 - 2$ | all four roots | Yes |
| $\mathbb{Q}(i)/\mathbb{Q}$ | $x^2 + 1$ | $i, -i$ | Yes |

## Proof Sketch

1. **Splitting fields are normal.** Let $K$ be the splitting field of $p$ over $F$. Take any irreducible $f \in F[x]$ with a root $\alpha \in K$. There is an $F$-isomorphism $F(\alpha) \cong F[x]/(f)$. All roots of $f$ are conjugates of $\alpha$; each lies in $K$ because $K$ is generated over $F$ by all roots of $p$, and adjoining any root of $f$ stays within the algebraic closure available. A careful argument using the universal property shows $f$ splits in $K$.
2. **Converse.** If $K/F$ is normal and finite, then $K$ is the splitting field of the product of the minimal polynomials of a generating set.

## Connections

Every normal separable extension is Galois, linking normality directly to the [[Fundamental Theorem of Galois Theory]]. The classification of which extensions are normal also underlies the [[Impossibility of the Quintic Formula]] — the key obstruction is that certain degree-5 extensions are not normal.

## Lean4 Proof

```lean4
/-- The splitting field of any polynomial is normal over the base field.
    Mathlib: `Normal.of_isSplittingField` with the `IsSplittingField` instance
    provided by `SplittingField`. -/
theorem splittingField_normal (F : Type*) [Field F]
    (p : Polynomial F) : Normal F (p.SplittingField) :=
  @Normal.of_isSplittingField F _ p.SplittingField _ _ p inferInstance
```
