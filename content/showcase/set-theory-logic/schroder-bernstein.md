+++
title = "Schröder–Bernstein Theorem"
description = "If there are injections A → B and B → A, then A and B have equal cardinality."
weight = 137
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "|A| \\le |B| \\land |B| \\le |A| \\implies |A| = |B|"
prerequisites = ["cantor-theorem"]
lean4_status = "complete"
+++

## Statement

If there exist injections $f : A \hookrightarrow B$ and $g : B \hookrightarrow A$, then $A$ and $B$ are in bijection:

$$|A| \le |B| \land |B| \le |A| \implies |A| = |B|$$

This allows us to prove two sets have the same cardinality without exhibiting the bijection directly.

## Visualization

Explicit bijection $\mathbb{N} \leftrightarrow \mathbb{Z}$ constructed from the two injections $f(n) = n$ and $g(z) = 2|z| + [z < 0]$.

```
ℕ  →  ℤ  (the explicit bijection h)

0  →  0
1  →  1
2  → -1
3  →  2
4  → -2
5  →  3
6  → -3
…

Pattern: h(2k)   =  k    (k ≥ 0)
         h(2k+1) = -(k+1)
```

The Schröder–Bernstein machinery builds $h$ by deciding for each element whether it falls in a "chain" traced back through $g \circ f$ to a point with no preimage under $g$, or in a cycle. In this example, the bijection comes out cleanly as the interleaving pattern above.

## Proof Sketch

1. For $x \in A$, define its **ancestry chain** by applying $g^{-1}$, then $f^{-1}$, then $g^{-1}$, and so on as long as possible.
2. Call $x$ a **$g$-origin** if its chain terminates (reaches a point with no $g$-preimage in $B$).
3. Define: $h(x) = f(x)$ if $x$ is a $g$-origin; otherwise $h(x) = g^{-1}(x)$.
4. Verify $h$ is a well-defined bijection. Injectivity and surjectivity follow from the chain analysis.

## Connections

The theorem is the cardinality analogue of [[Heine–Borel Theorem|antisymmetry]] in orders. It underlies the fact that cardinals are totally ordered (given injections in both directions, the sets are equinumerous). Compare with [[Cantor's Theorem]], which shows strict inequality when no surjection exists.

## Lean4 Proof

```lean4
import Mathlib.SetTheory.Cardinal.SchroederBernstein

/-- Schröder–Bernstein: injections in both directions yield a bijection.
    Mathlib: `schroeder_bernstein`. -/
theorem schroder_bernstein_theorem {α β : Type*}
    {f : α → β} {g : β → α}
    (hf : Function.Injective f) (hg : Function.Injective g) :
    ∃ h : α → β, Function.Bijective h :=
  schroeder_bernstein hf hg
```
