+++
title = "Representable Functor"
description = "A functor F is representable if it is naturally isomorphic to Hom(X,-) for some object X"
weight = 60
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "F \\cong \\mathrm{Hom}(X, -)"
prerequisites = ["yoneda-lemma", "natural-transformation"]
lean4_status = "complete"
+++

## Statement

A functor $F : \mathcal{C} \to \mathbf{Set}$ is **representable** if there exists an object $X \in \mathcal{C}$ and a natural isomorphism:

$$F \;\cong\; \mathrm{Hom}(X, -)$$

The object $X$ is called the **representing object**. By the Yoneda Lemma, such an isomorphism corresponds to a distinguished element $u \in F(X)$ (the **universal element**): every element of $F(A)$ arises uniquely as $F(f)(u)$ for some $f : X \to A$.

Dually, $F : \mathcal{C}^{\mathrm{op}} \to \mathbf{Set}$ is representable if $F \cong \mathrm{Hom}(-, X)$.

## Visualization

**$\mathrm{Hom}(\mathbb{Z}, -)$ represents the underlying-set functor on $\mathbf{Ab}$:**

```
  In category Ab (abelian groups):

  Hom(ℤ, A) ≅ |A|  (underlying set of A)

  The bijection sends f : ℤ → A to f(1) ∈ A.
  Inverse: given a ∈ A, define f_a(n) = n·a.

  Example with A = ℤ/6ℤ:
  ┌────────────────────────────────────────────┐
  │  Hom(ℤ, ℤ/6ℤ)  ←→  {0,1,2,3,4,5}         │
  │  f₀ : n ↦ 0·n          ←→  0              │
  │  f₁ : n ↦ n mod 6      ←→  1              │
  │  f₂ : n ↦ 2n mod 6     ←→  2              │
  │  f₃ : n ↦ 3n mod 6     ←→  3              │
  │  f₄ : n ↦ 4n mod 6     ←→  4              │
  │  f₅ : n ↦ 5n mod 6     ←→  5              │
  └────────────────────────────────────────────┘

  Universal element: u = 1 ∈ ℤ  (the identity map id_ℤ corresponds to 1)

  Naturality: for φ : ℤ/6ℤ → ℤ/6ℤ, g ↦ φ(g(1)) = (φ∘g)(1)  ✓
```

## Proof Sketch

1. **Yoneda gives the bijection:** By the Yoneda Lemma, $\mathrm{Nat}(\mathrm{Hom}(X,-), F) \cong F(X)$. A representing object is one for which this set of natural isomorphisms is non-empty and the iso picks out the tautological one.

2. **The universal element characterises $X$:** The element $u = \alpha_X(\mathrm{id}_X) \in F(X)$ for the natural isomorphism $\alpha$ is universal: for any $A$ and any $a \in F(A)$, there is a unique $f : X \to A$ with $F(f)(u) = a$.

3. **Representing objects are unique up to isomorphism:** If $(X, u)$ and $(X', u')$ both represent $F$, the universal property gives morphisms $X \to X'$ and $X' \to X$ that compose to identities.

4. **The yoneda embedding is fully faithful:** This means $\mathrm{Hom}(X, Y) \cong \mathrm{Nat}(\mathrm{Hom}(-,X), \mathrm{Hom}(-,Y))$, so representable functors determine and are determined by their objects.

## Connections

Representability is the conceptual core of the [[Yoneda Lemma]]. The universal property of free objects in [[Adjoint Functors]] (free monoid, free group, etc.) can be phrased as: the underlying-set functor is representable, with the free object as representing object. Limits are also representable: $\mathrm{Hom}(X, \varprojlim D) \cong \mathrm{Lim}_j \mathrm{Hom}(X, D(j))$ — see [[Limits and Colimits]].

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Yoneda

open CategoryTheory

/-- The presheaf yoneda.obj X is representable (by X itself).
    Mathlib: the instance `IsRepresentable (yoneda.obj X)`. -/
theorem yoneda_obj_is_representable {C : Type*} [Category C] (X : C) :
    (yoneda.obj X).IsRepresentable :=
  inferInstance

/-- Any presheaf isomorphic to a representable is representable.
    Use `IsRepresentable.mk'` which takes a natural isomorphism witness. -/
theorem representable_of_iso {C : Type*} [Category C]
    {F : Cᵒᵖ ⥤ Type*} {X : C} (e : yoneda.obj X ≅ F) :
    F.IsRepresentable :=
  Functor.IsRepresentable.mk' e
```
