+++
title = "Yoneda Lemma"
description = "Natural transformations from a representable functor y_X to F biject with elements of F(X)"
weight = 10
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "\\mathrm{Nat}(\\mathrm{Hom}(X,-),\\, F) \\cong F(X)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $\mathcal{C}$ be a locally small category, $X \in \mathcal{C}$ an object, and $F : \mathcal{C}^{\mathrm{op}} \to \mathbf{Set}$ a presheaf. The **Yoneda Lemma** states there is a natural bijection:

$$\mathrm{Nat}(\mathrm{Hom}(-, X),\, F) \;\cong\; F(X)$$

The bijection sends a natural transformation $\alpha : \mathrm{Hom}(-,X) \Rightarrow F$ to the element $\alpha_X(\mathrm{id}_X) \in F(X)$. Its inverse sends $x \in F(X)$ to the natural transformation $(\alpha_x)_Y(f) = F(f)(x)$.

## Visualization

Concretely, take $\mathcal{C} = \mathbf{Set}$, $X = \{*\}$ (one-element set), $F = \mathcal{P}$ (power-set functor on $\mathbf{Set}^{\mathrm{op}}$). Then $\mathrm{Hom}(-, \{*\})$ picks out elements, and a natural transformation corresponds to a subset of $\{*\}$:

```
  Naturality square for α : Hom(-,X) ⇒ F, and f : A → B
                            α_B
  Hom(B, X) ─────────────────────────────▶ F(B)
       │                                      │
  -∘f  │                                      │ F(f)
       ▼                                      ▼
  Hom(A, X) ─────────────────────────────▶ F(A)
                            α_A

  α commutes: F(f)(α_B(g)) = α_A(g ∘ f)  for all g : B → X

  Key computation (recovering x from α):
    given α, set x := α_X(id_X) ∈ F(X)
    then for any f : A → X:
      α_A(f) = α_A(id_X ∘ f) = F(f)(α_X(id_X)) = F(f)(x)  ✓
```

So $\alpha$ is completely determined by the single element $x = \alpha_X(\mathrm{id}_X)$.

## Proof Sketch

1. **Define the map $\Phi$:** Given $\alpha : \mathrm{Hom}(-,X) \Rightarrow F$, set $\Phi(\alpha) = \alpha_X(\mathrm{id}_X) \in F(X)$.

2. **Define the inverse $\Psi$:** Given $x \in F(X)$, define $\Psi(x)_A(f) = F(f)(x)$ for $f : A \to X$. Naturality of $\Psi(x)$ follows from functoriality of $F$.

3. **$\Phi \circ \Psi = \mathrm{id}$:** $\Phi(\Psi(x)) = \Psi(x)_X(\mathrm{id}_X) = F(\mathrm{id}_X)(x) = x$. Since $F$ is a functor, $F(\mathrm{id}_X) = \mathrm{id}_{F(X)}$.

4. **$\Psi \circ \Phi = \mathrm{id}$:** For $\alpha$ natural and any $f : A \to X$, naturality forces $\alpha_A(f) = F(f)(\alpha_X(\mathrm{id}_X)) = \Psi(\Phi(\alpha))_A(f)$.

5. **Naturality of $\Phi$:** Both bijections vary naturally in $X$ and $F$, so $\Phi$ is a natural isomorphism.

## Connections

The Yoneda Lemma is the categorical generalization of [[Cayley's Theorem]] (every group embeds in its permutation group) and underlies the theory of [[Representable Functor|representable functors]]. It also drives the universal properties behind [[Adjoint Functors]].

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Yoneda

open CategoryTheory

/-- The Yoneda bijection: natural transformations from the representable
    presheaf yoneda.obj X to F correspond to elements of F.obj (op X).
    Mathlib provides this as `yonedaEquiv`. -/
theorem yoneda_bijection {C : Type*} [Category C]
    {X : C} {F : Cᵒᵖ ⥤ Type*} :
    (yoneda.obj X ⟶ F) ≃ F.obj (Opposite.op X) :=
  yonedaEquiv

/-- The underlying element recovered from a natural transformation. -/
theorem yoneda_apply {C : Type*} [Category C]
    {X : C} {F : Cᵒᵖ ⥤ Type*} (α : yoneda.obj X ⟶ F) :
    yonedaEquiv α = α.app (Opposite.op X) (𝟙 X) :=
  yonedaEquiv_apply α
```
