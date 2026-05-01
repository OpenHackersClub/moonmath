+++
title = "Linearity of Conditional Expectation"
description = "Conditional expectation is linear: E[aX+bY|G] = aE[X|G] + bE[Y|G] almost surely"
weight = 100
tags = ["lean4-proof", "probability", "visualization"]
latex = "E[aX + bY \\mid \\mathcal{G}] = a\\,E[X \\mid \\mathcal{G}] + b\\,E[Y \\mid \\mathcal{G}]"
prerequisites = ["total-expectation"]
lean4_status = "complete"
+++

## Statement

Let $\mathcal{G} \subseteq \mathcal{F}$ be a sub-$\sigma$-algebra, and let $X$, $Y$ be integrable real random variables. For constants $a, b \in \mathbb{R}$:

$$E[aX + bY \mid \mathcal{G}] = a\,E[X \mid \mathcal{G}] + b\,E[Y \mid \mathcal{G}] \quad \text{a.s.}$$

This follows from two atomic properties:
- **Additivity:** $E[X + Y \mid \mathcal{G}] = E[X \mid \mathcal{G}] + E[Y \mid \mathcal{G}]$ a.s.
- **Scalar multiplication:** $E[cX \mid \mathcal{G}] = c\,E[X \mid \mathcal{G}]$ a.s.

## Visualization

**Two dice experiment.** Roll dice $A$ and $B$ independently. Let $X = A$, $Y = B$, and $\mathcal{G} = \sigma(A)$ (information from die $A$ only).

| Outcome $A=a$ | $E[X \mid A=a]$ | $E[Y \mid A=a]$ | $E[2X + 3Y \mid A=a]$ | Direct: $2a + 3 \cdot 3.5$ |
|--------------|----------------|----------------|----------------------|---------------------------|
| $a = 1$      | $1$            | $3.5$          | $2 + 10.5 = 12.5$    | $2(1) + 10.5 = 12.5$ |
| $a = 3$      | $3$            | $3.5$          | $6 + 10.5 = 16.5$    | $2(3) + 10.5 = 16.5$ |
| $a = 6$      | $6$            | $3.5$          | $12 + 10.5 = 22.5$   | $2(6) + 10.5 = 22.5$ |

Since $B$ is independent of $A$, $E[Y \mid A=a] = E[Y] = 3.5$ for all $a$. Linearity gives $E[2X + 3Y \mid A] = 2A + 10.5$ a.s., matching direct computation.

## Proof Sketch

1. **Additivity.** Show that $E[X \mid \mathcal{G}] + E[Y \mid \mathcal{G}]$ satisfies the defining property of $E[X+Y \mid \mathcal{G}]$: it is $\mathcal{G}$-measurable and for all $A \in \mathcal{G}$, $\int_A (E[X|\mathcal{G}] + E[Y|\mathcal{G}]) \, dP = \int_A X \, dP + \int_A Y \, dP = \int_A (X+Y) \, dP$.
2. **Uniqueness.** Conditional expectation is defined up to a.s. equality by the defining property, so additivity holds a.s.
3. **Scalar.** $c \cdot E[X \mid \mathcal{G}]$ is $\mathcal{G}$-measurable and $\int_A c \cdot E[X|\mathcal{G}] \, dP = c \int_A X \, dP$ by linearity of integration.

## Connections

Linearity of conditional expectation is the key structural property underpinning the [[Law of Total Expectation]] and the martingale property in [[Martingale Definition]]. Combined with [[Jensen's Inequality]] (for convex $\varphi$: $\varphi(E[X|\mathcal{G}]) \leq E[\varphi(X)|\mathcal{G}]$ a.s.), it characterises the relationship between conditional and unconditional moments.

## Lean4 Proof

Mathlib provides `condExp_add` and `condExp_smul` in `Mathlib.MeasureTheory.Function.ConditionalExpectation.Basic`.

```lean4
import Mathlib.MeasureTheory.Function.ConditionalExpectation.Basic

namespace MoonMath

open MeasureTheory

/-- **Additivity of conditional expectation**.
    `μ[f + g | m] =ᵐ[μ] μ[f | m] + μ[g | m]`. -/
theorem condExp_add_ae {α : Type*} {m₀ m : MeasurableSpace α} (μ : Measure α)
    {f g : α → ℝ} (hf : Integrable f μ) (hg : Integrable g μ) :
    μ[fun x => f x + g x | m] =ᵐ[μ] fun x => (μ[f | m]) x + (μ[g | m]) x :=
  condExp_add hf hg m

/-- **Homogeneity of conditional expectation**.
    `μ[c • f | m] =ᵐ[μ] c • μ[f | m]`. -/
theorem condExp_smul_ae {α : Type*} {m₀ m : MeasurableSpace α} (μ : Measure α)
    (c : ℝ) (f : α → ℝ) :
    μ[fun x => c * f x | m] =ᵐ[μ] fun x => c * (μ[f | m]) x := by
  have := condExp_smul (𝕜 := ℝ) c f m (α := α) (μ := μ)
  simpa [smul_eq_mul] using this

end MoonMath
```

`condExp_add` is proved in Mathlib by showing the sum satisfies the defining characterisation of conditional expectation; `condExp_smul` follows from linearity of the Bochner integral.

