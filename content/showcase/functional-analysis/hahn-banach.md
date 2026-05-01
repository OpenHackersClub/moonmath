+++
title = "Hahn–Banach Theorem"
description = "A bounded linear functional on a subspace of a normed space extends to the whole space with the same norm"
weight = 10
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "\\exists\\, g : E \\to \\mathbb{R},\\; g|_p = f,\\; \\|g\\| = \\|f\\|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $E$ be a normed space over $\mathbb{R}$ (or $\mathbb{C}$), $p \subseteq E$ a subspace, and $f : p \to \mathbb{R}$ a bounded linear functional. Then there exists a bounded linear functional $g : E \to \mathbb{R}$ such that

$$g\big|_p = f \quad \text{and} \quad \|g\| = \|f\|.$$

The extension preserves the norm exactly — it does not "inflate" the functional. This is the analytic (norm-extension) form; the geometric form gives separating hyperplanes for convex sets.

## Visualization

Extend $f(t) = 3t$ defined on the subspace $p = \mathbb{R} \cdot e_1 \subseteq \mathbb{R}^2$:

```
  E = ℝ²         p = {(t, 0) : t ∈ ℝ}

  e₂ axis
  │
  │   *  ← (1,1) in E, no f-value on p
  │
──┼───────────────────▶ e₁ axis
  │   ←  p = ℝ·e₁  →
  │   f(t,0) = 3t

One extension: g(x₁, x₂) = 3x₁  (ignores x₂ component)
  g(1,0) = 3 = f(1,0)  ✓
  g(0,1) = 0           (free choice, norm preserved)
  ‖g‖ = 3 = ‖f‖        ✓
```

| Point in $p$ | $f$ value | $g$ value (extension) |
|---|---|---|
| $(1, 0)$ | $3$ | $3$ |
| $(2, 0)$ | $6$ | $6$ |
| $(-1, 0)$ | $-3$ | $-3$ |
| $(0, 1) \notin p$ | — | $0$ (chosen) |
| $(1, 1) \notin p$ | — | $3$ (chosen) |

The extension $g(x_1, x_2) = 3x_1$ has $\|g\| = 3 = \|f\|$, confirming the norm is not inflated.

## Proof Sketch

1. **Seminorm domination.** Let $p(x) = \|f\| \cdot \|x\|$ be a seminorm on $E$ dominating $f$ on $p$.
2. **One-dimensional extension.** For any $x_0 \notin p$, extend $f$ to $p + \mathbb{R} x_0$ by choosing $g(x_0)$ so that $g \le p$ everywhere. Algebraic manipulation shows a valid choice exists.
3. **Zorn's lemma (or transfinite induction).** Among all extensions dominated by $p$, take a maximal one. Maximality forces the domain to be all of $E$.
4. **Norm equality.** $\|g\| \le \|f\|$ by domination; $\|g\| \ge \|f\|$ because $g$ agrees with $f$ on $p$.

## Connections

- [[Cauchy–Schwarz Inequality]] — the inner product form of the Hahn–Banach bound in Hilbert spaces reduces to Cauchy–Schwarz.
- [[Riesz Representation Theorem]] — in Hilbert spaces, Hahn–Banach is superseded by the Riesz theorem, which identifies the extending functional with an inner product.
- [[Spectral Radius Formula]] — dual space techniques from Hahn–Banach underpin the analytic function proof of the Gelfand formula.
- [[Heine–Borel Theorem]] — compactness and weak-* compactness (Banach–Alaoglu, related to Hahn–Banach) parallel the finite-dimensional Heine–Borel picture.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Module.HahnBanach

/-- **Hahn–Banach theorem** (real normed space, norm-preserving extension).
    Direct alias of `exists_extension_norm_eq` in Mathlib. -/
theorem hahn_banach_extension (E : Type*) [SeminormedAddCommGroup E] [NormedSpace ℝ E]
    (p : Subspace ℝ E) (f : StrongDual ℝ p) :
    ∃ g : StrongDual ℝ E, (∀ x : p, g x = f x) ∧ ‖g‖ = ‖f‖ :=
  exists_extension_norm_eq p f
```
