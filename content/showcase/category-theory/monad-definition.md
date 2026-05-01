+++
title = "Monad"
description = "A monad is a functor T with unit η and multiplication μ satisfying associativity and unit coherence"
weight = 80
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "\\mu \\circ T\\mu = \\mu \\circ \\mu T \\quad \\mu \\circ T\\eta = \\mathrm{id} = \\mu \\circ \\eta T"
prerequisites = ["adjoint-functors", "natural-transformation", "functor-composition"]
lean4_status = "complete"
+++

## Statement

A **monad** on a category $\mathcal{C}$ is a triple $(T, \eta, \mu)$ consisting of:
- A functor $T : \mathcal{C} \to \mathcal{C}$ (the underlying endofunctor)
- A natural transformation $\eta : \mathrm{Id}_\mathcal{C} \Rightarrow T$ (the **unit**)
- A natural transformation $\mu : T^2 \Rightarrow T$ (the **multiplication**, $T^2 = T \circ T$)

satisfying the **monad laws** (commutativity of these diagrams):
- **Associativity:** $\mu \circ T\mu = \mu \circ \mu T$ (as natural transformations $T^3 \Rightarrow T$)
- **Left unit:** $\mu \circ \eta T = \mathrm{id}_T$
- **Right unit:** $\mu \circ T\eta = \mathrm{id}_T$

## Visualization

**List monad on Set** (or Type in Lean 4):

```
  T(A)  = List A   (finite lists of elements of A)
  η_A   : A → List A,        a ↦ [a]       (singleton)
  μ_A   : List(List A) → List A,  [[a,b],[c]] ↦ [a,b,c]  (flatten)

  Coherence checks:
  ┌──────────────────────────────────────────────────────────┐
  │  Associativity:  μ ∘ Tμ = μ ∘ μT                        │
  │                                                          │
  │  Input: [[[1,2],[3]], [[4]]]  (List(List(List ℕ)))       │
  │                                                          │
  │  μT applied first (flatten inner):                       │
  │    [[[1,2],[3]], [[4]]] ──▶ [[1,2,3],[4]] ──▶ [1,2,3,4] │
  │                                                          │
  │  Tμ applied first (map flatten):                         │
  │    [[[1,2],[3]], [[4]]] ──▶ [[1,2,3],[4]] ──▶ [1,2,3,4] │
  │                                    same result!  ✓       │
  │                                                          │
  │  Left unit:  μ ∘ ηT = id                                 │
  │    [1,2,3] ──η_T([1,2,3])──▶ [[1,2,3]] ──μ──▶ [1,2,3]  │
  │                                                          │
  │  Right unit:  μ ∘ Tη = id                               │
  │    [1,2,3] ──T(η)──▶ [[1],[2],[3]] ──μ──▶ [1,2,3]  ✓   │
  └──────────────────────────────────────────────────────────┘
```

## Proof Sketch

1. **Every adjunction gives a monad:** If $L \dashv R$, then $T = RL$, $\eta : \mathrm{Id} \Rightarrow RL$ (unit of adjunction), $\mu = R\varepsilon L : RLRL \Rightarrow RL$ (where $\varepsilon$ is the counit). The monad laws follow from the triangle identities of the adjunction.

2. **List monad from free-monoid adjunction:** The free-monoid functor $F \dashv U$ (forget) gives $T = UF = $ List. The unit $\eta_A(a) = [a]$ comes from the adjunction unit; multiplication $\mu = U\varepsilon F$ is the concatenation/flatten operation.

3. **Monad laws for List:** Associativity of List flatten is the standard `List.join_join` identity. Unit laws are `List.join_singleton` and `List.join_map_singleton`.

4. **Kleisli category:** A monad determines a Kleisli category whose morphisms $A \to B$ are functions $A \to TB$ composed via $\mu$ — this is the "monadic bind" familiar from Haskell.

## Connections

Every adjunction generates a monad via $T = RL$ — the converse (Eilenberg–Moore and Kleisli) requires additional structure. The list monad is the free-monoid monad; the power-set monad corresponds to the adjunction between $\mathbf{Set}$ and $\mathbf{Rel}$. Monads in programming languages (Haskell, Lean 4's `do`-notation) are instances of this categorical structure. See also [[Adjoint Functors]] and the [[Yoneda Lemma]] (the Yoneda embedding is itself a representation of a monad-like structure).

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Monad.Basic
import Mathlib.CategoryTheory.Monad.Types

open CategoryTheory

/-- Every Lean/Haskell-style monad m gives a categorical monad on Type.
    Mathlib: `CategoryTheory.Monad.ofTypeMonad` in Monad/Types.lean. -/
theorem list_gives_categorical_monad :
    Nonempty (Monad (Type 0)) :=
  ⟨CategoryTheory.Monad.ofTypeMonad List⟩

/-- The unit law for the list monad: flatten ∘ singleton = id. -/
theorem list_monad_left_unit (α : Type) (as : List α) :
    List.join (List.map (fun a => [a]) as) = as := by
  simp [List.join_map_singleton]

/-- The associativity law for the list monad: flatten ∘ flatten = flatten ∘ map_flatten. -/
theorem list_monad_assoc (α : Type) (xss : List (List (List α))) :
    List.join (List.join xss) = List.join (List.map List.join xss) := by
  simp [List.join_join]
```
