+++
title = "Limits and Colimits"
description = "Universal cones over diagrams: limits generalize products and equalizers; colimits generalize coproducts and coequalizers"
weight = 30
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "\\varprojlim D \\dashv \\Delta \\dashv \\varinjlim D"
prerequisites = ["adjoint-functors", "natural-transformation"]
lean4_status = "complete"
+++

## Statement

Let $J$ be a small category (the **shape**) and $D : J \to \mathcal{C}$ a **diagram** in $\mathcal{C}$. A **limit** of $D$ is an object $\varprojlim D$ together with projection morphisms $\pi_j : \varprojlim D \to D(j)$ that form a cone, universal among all such cones:

$$\forall \text{ cone } (X, \phi),\; \exists! \text{ mediating morphism } X \to \varprojlim D$$

A **colimit** $\varinjlim D$ is the dual: a universal cocone with injection morphisms $\iota_j : D(j) \to \varinjlim D$.

The category $\mathbf{Type}$ has all small limits (instance `hasLimitsOfSize` in Mathlib).

## Visualization

**Pullback (limit of a cospan) in Set:**

```
  Diagram:  A ──f──▶ C ◀──g── B    (shape: • → • ← •)

  Limit = A ×_C B = { (a, b) ∈ A × B | f(a) = g(b) }

  Example: A = {1,2,3}, B = {x,y,z}, C = {red,blue}
           f(1)=f(2)=red, f(3)=blue
           g(x)=red, g(y)=g(z)=blue

  A ×_C B = { (1,x), (2,x), (3,y), (3,z) }

  Universal cone:
                 A ×_C B
               ↙         ↘
             A              B
              ↘            ↙
                  C

  Any cone W ──▶ A, W ──▶ B with f∘π_A = g∘π_B
  factors uniquely through A ×_C B.
```

**Equalizer (limit of a parallel pair):**

```
  Diagram:  A ═══f═══▶ B
               ═══g═══▶

  Equalizer = Eq(f,g) = { a ∈ A | f(a) = g(a) }

  f(x) = x², g(x) = 2x on ℤ:  Eq(f,g) = {0, 2}  (since 0²=0, 2²=4=2·2)
```

## Proof Sketch

1. **Existence of limits in Set:** Given $D : J \to \mathbf{Set}$, form $\varprojlim D = \{(x_j)_{j \in J} \in \prod_j D(j) \mid D(f)(x_j) = x_k \text{ for all } f : j \to k\}$. The projections $\pi_j((x_k)_k) = x_j$ are natural.

2. **Universality:** Any cone $(X, \phi)$ gives a unique map $X \to \varprojlim D$ by $x \mapsto (\phi_j(x))_{j \in J}$; the compatibility condition $D(f)(\phi_j(x)) = \phi_k(x)$ ensures the tuple lands in $\varprojlim D$.

3. **Colimits via sets and quotients:** $\varinjlim D = \bigsqcup_j D(j) / {\sim}$ where $x \sim D(f)(x)$ for all morphisms $f$ in $J$.

4. **Adjunction characterization:** Limits and colimits are right and left adjoints of the diagonal functor $\Delta : \mathcal{C} \to \mathcal{C}^J$.

## Connections

Limits and colimits generalize the [[Fundamental Theorem of Arithmetic]] (GCDs and LCMs are limits/colimits in the divisibility poset). The adjunction $\varprojlim \dashv \Delta \dashv \varinjlim$ is a fundamental instance of [[Adjoint Functors]].

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Limits.Types.Limits

open CategoryTheory CategoryTheory.Limits

/-- The category Type u has limits of all small diagrams.
    Mathlib: `hasLimitsOfSize` instance in CategoryTheory.Limits.Types.Limits. -/
theorem type_has_limits : HasLimitsOfSize.{0, 0} (Type 0) :=
  inferInstance

/-- Concretely, a limit in Type is a compatible section of the diagram. -/
example {J : Type} [SmallCategory J] (D : J ⥤ Type 0) :
    HasLimit D :=
  inferInstance
```
