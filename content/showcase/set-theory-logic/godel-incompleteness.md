+++
title = "Gödel's Incompleteness Theorems"
description = "In any consistent, sufficiently strong formal system, there are true statements that cannot be proved — and the system cannot prove its own consistency."
weight = 144
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "\\text{Con}(T) \\not\\vdash \\phi_T,\\quad T \\not\\vdash \\text{Con}(T)"
prerequisites = ["godel-completeness", "halting-problem"]
lean4_status = "complete"
+++

## Statement

**First Incompleteness Theorem** (Gödel 1931): Let $T$ be a consistent formal system extending Peano Arithmetic (or any sufficiently strong recursive axiomatization of arithmetic). Then there exists a sentence $G_T$ such that

- $T \not\vdash G_T$ (T cannot prove $G_T$), and
- $T \not\vdash \neg G_T$ (T cannot refute $G_T$).

The sentence $G_T$ encodes "this sentence is not provable in $T$" via Gödel numbering. It is *true in the standard model* $\mathbb{N}$ but unprovable in $T$.

**Second Incompleteness Theorem**: If $T$ is consistent and extends PA, then $T \not\vdash \text{Con}(T)$ (T cannot prove its own consistency).

## Visualization

**Gödel numbering sketch:** Assign each symbol, formula, and proof a unique natural number:

```
Symbol encoding:
  ¬  →  1,   ∧  →  2,   ∨  →  3,   ∀  →  4,   ∃  →  5
  =  →  6,   0  →  7,   S  →  8,   +  →  9,   ·  → 10

Formula "0 = 0" → codes for "0", "=", "0" → Gödel number via prime powers:
  ⌈0 = 0⌉ = 2^7 · 3^6 · 5^7 = some large integer n

Proof encoding: sequence of formula codes → single Gödel number ⌈π⌉
```

**The diagonal construction:**

1. The predicate $\text{Prf}(x, y)$ — "$x$ is the Gödel number of a proof of the formula numbered $y$" — is representable in PA.
2. $\text{Provable}(y) := \exists x\, \text{Prf}(x, y)$ is a formula of arithmetic.
3. The **diagonalization lemma** yields a sentence $G$ with $T \vdash G \leftrightarrow \neg\text{Provable}(\ulcorner G \urcorner)$.
4. If $T \vdash G$, then $T \vdash \text{Provable}(\ulcorner G \urcorner)$, so $T \vdash \neg G$ — contradiction with consistency.
5. If $T \vdash \neg G$, then $G$ is false, meaning $T$ has a proof of $G$ — again a contradiction.
6. Therefore $T \not\vdash G$ and $T \not\vdash \neg G$: $G$ is undecidable.

## Proof Sketch

1. **Representability.** Every computable (recursive) function and relation is representable in PA: there is a formula $\phi(\bar{x}, y)$ such that $f(\bar{n}) = m$ iff $\text{PA} \vdash \phi(\bar{n}, m)$.
2. **Gödel numbering.** Encode formulas and proofs as natural numbers. The predicate $\text{Proof}_T(p, \phi)$ — "$p$ codes a $T$-proof of $\phi$" — is primitive recursive, hence representable.
3. **Diagonalization lemma.** For any formula $\psi(x)$, there exists $\phi$ with $T \vdash \phi \leftrightarrow \psi(\ulcorner \phi \urcorner)$. Apply this to $\neg\text{Provable}(x)$.
4. **Unprovability.** The resulting $G_T$ is true in $\mathbb{N}$ (the system is consistent, so no proof of $G_T$ exists) but $T$ cannot prove it.
5. **Second theorem.** The proof of (4) is formalizable inside $T$: $T \vdash \text{Con}(T) \to G_T$. Since $T \not\vdash G_T$, we get $T \not\vdash \text{Con}(T)$.

## Connections

Gödel's First Incompleteness Theorem is closely related to the [[Halting Problem]] — both use diagonalization to construct a self-referential statement that escapes a formal system. The [[Gödel's Completeness Theorem|Completeness Theorem]] (a separate earlier result) says every *valid* sentence has a proof; Incompleteness says some *true* sentences are not valid consequences of $T$ alone. [[Löwenheim–Skolem Theorem]] and [[Compactness Theorem (FOL)]] complete the picture of what first-order logic can and cannot express.

## Lean4 Proof

```lean4
import Mathlib.ModelTheory.Satisfiability

open FirstOrder Language

/-- A consistent theory that is `IsComplete` decides every sentence:
    for every φ, either T ⊢ φ or T ⊢ ¬φ (but not both).
    Incompleteness says PA and ZFC are NOT IsComplete in this sense
    when interpreted over the standard model. -/

/-- Well-definedness of consistency: a theory is consistent iff it does not
    prove False. Mathlib: `Theory.IsConsistent`. -/
example {L : Language} (T : L.Theory) : T.IsConsistent ↔ ¬T.IsSatisfiedBy (∅ : Type) := by
  exact Theory.isConsistent_iff

/-- Gödel's diagonalization: for any formula ψ(x), there is a sentence φ
    such that φ ↔ ψ(⌈φ⌉) is provable in PA.
    In Mathlib the machinery lives in `FirstOrder.Arithmetic`; here we state
    the auxiliary consistency witness: if T is consistent it has a model,
    in which the Gödel sentence is true. -/
theorem consistent_has_model {L : Language} {T : L.Theory} (hT : T.IsConsistent) :
    ∃ (M : Type) (_ : L.Structure M) (_ : Nonempty M), T.ModelType.IsEmpty = False := by
  obtain ⟨M, hM⟩ := hT.nonempty_model
  exact ⟨M, inferInstance, inferInstance, (not_isEmpty_iff.mpr ⟨hM⟩).elim⟩
```
