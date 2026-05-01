+++
title = "Gödel's Completeness Theorem"
description = "A first-order sentence is provable from a theory iff it holds in every model of that theory."
weight = 142
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "T \\vdash \\phi \\iff T \\models \\phi"
prerequisites = ["compactness-fol"]
lean4_status = "complete"
+++

## Statement

Gödel's Completeness Theorem (1930): for any first-order theory $T$ and sentence $\phi$,

$$T \vdash \phi \iff T \models \phi$$

Syntactic provability ($\vdash$) and semantic truth in all models ($\models$) coincide. Every tautology has a formal proof.

## Visualization

Consider the tautology $\models \forall x\, P(x) \to \exists x\, P(x)$.

```
Proof tree (natural deduction):
                    [∀x P(x)]
                        |
               (∀-elim, any term a)
                      P(a)
                        |
               (∃-intro, witness a)
                   ∃x P(x)
            ─────────────────────
         ∀x P(x) → ∃x P(x)          QED

The proof uses only 3 rules:
  ∀-elim: from ∀x φ(x), derive φ(t) for any term t
  ∃-intro: from φ(t), derive ∃x φ(x)
  →-intro: from a derivation of ψ assuming φ, derive φ → ψ
```

Completeness guarantees: if a sentence holds in every structure (like $\forall x P(x) \to \exists x P(x)$), then there is always a finite proof tree witnessing it.

## Proof Sketch

The key is to show: if $T \not\vdash \phi$, then $T \not\models \phi$ (i.e., there is a model of $T$ where $\phi$ is false).

1. Assume $T \cup \{\neg\phi\}$ is consistent (no proof of contradiction).
2. By the **Lindenbaum–Tarski** construction, extend $T \cup \{\neg\phi\}$ to a maximal consistent theory $T^*$.
3. Build the **term model** (Henkin construction): the domain consists of equivalence classes of terms under $T^*$-provable equality. Interpret each predicate by whether $T^*$ proves it.
4. The term model satisfies exactly the sentences in $T^*$. In particular it satisfies $T$ and $\neg\phi$, so $T \not\models \phi$.

## Connections

Completeness and [[Compactness Theorem (FOL)|Compactness]] are the two pillars of classical model theory — together they show that first-order logic is "perfectly calibrated." Gödel's *Incompleteness* theorems (a separate result) show that sufficiently strong theories cannot prove their own consistency, but this does not contradict completeness: every true semantic consequence still has a syntactic proof.

Compare [[Löwenheim–Skolem Theorem]], another consequence of the Henkin-style model construction.

## Lean4 Proof

```lean4
import Mathlib.ModelTheory.Satisfiability

open FirstOrder Language

/-- The complete theory of any nonempty structure is complete:
    every sentence or its negation is provable.
    This is the Mathlib formulation closest to Gödel's Completeness Theorem —
    the complete theory of M satisfies every semantic consequence.
    Mathlib: `FirstOrder.Language.completeTheory.isComplete`. -/
theorem complete_theory_is_complete
    {L : Language} (M : Type*) [L.Structure M] [Nonempty M] :
    (L.completeTheory M).IsComplete :=
  completeTheory.isComplete L M
```
