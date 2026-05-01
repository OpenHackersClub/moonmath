+++
title = "Baire Category Theorem"
description = "In a complete metric space, the intersection of countably many dense open sets is dense"
weight = 140
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\bigcap_{n=1}^{\\infty} U_n \\text{ dense},\\quad U_n \\text{ open dense in complete metric space}"
prerequisites = ["heine-borel"]
lean4_status = "complete"
+++

Originally indexed under Analysis; canonical home is Topology since the result is a property of the space.

## Statement

Let $X$ be a complete metric space (or, more generally, a locally compact Hausdorff space). If $(U_n)_{n \ge 1}$ is a countable family of open dense subsets of $X$, then

$$\bigcap_{n=1}^{\infty} U_n \text{ is dense in } X.$$

Equivalently: a complete metric space is **not** a countable union of nowhere-dense sets. A set is **meager** (first category) if it is a countable union of nowhere-dense sets; Baire's theorem says no complete metric space is meager.

## Visualization

$\mathbb{Q}$ as countable union of nowhere-dense singletons — the irrationals are the dense residual:

```
ℝ (complete metric space):
────────────────────────────────────────────────────▶

ℚ  = {0} ∪ {1} ∪ {1/2} ∪ {1/3} ∪ ... (each singleton nowhere-dense)
     Meager (first category) — measure zero, topologically "small"

ℝ\ℚ = irrationals
     Dense residual (complement of ℚ is dense everywhere)

U_n = ℝ\{q_n}  (complement of the n-th rational, open and dense)

⋂ U_n = ℝ\ℚ = irrationals  ← dense!
n≥1

The Baire theorem guarantees ⋂ U_n is dense because each U_n is open and dense
in the complete metric space ℝ.  Nowhere does the intersection collapse to empty.
```

Nowhere-dense test for $\{q\}$: its closure is $\{q\}$, whose interior is empty. So each rational singleton is nowhere-dense, and $\mathbb{Q}$ is first-category, even though it is dense.

## Proof Sketch

1. **Goal:** show every non-empty open $V \subseteq X$ meets $\bigcap_n U_n$.
2. **Inductive construction:** $V$ meets $U_1$ (since $U_1$ is dense), so pick $x_1 \in V \cap U_1$ and an open ball $B_1 \subseteq V \cap U_1$ with $\overline{B_1}$ compact and radius $< 1$.
3. **Continue:** $B_1$ meets $U_2$; pick $B_2 \subseteq B_1 \cap U_2$ with radius $< 1/2$. At step $n$, $B_n \subseteq B_{n-1} \cap U_n$ with radius $< 1/n$.
4. **Cauchy sequence:** centers $x_n \in B_n$ form a Cauchy sequence because $\text{diam}(B_n) < 2/n$.
5. **Limit:** by completeness, $x_n \to x \in X$. Then $x \in \overline{B_n} \subseteq U_n$ for all $n$, and $x \in V$.
6. **Conclusion:** $x \in V \cap \bigcap_n U_n$, so the intersection is dense.

## Connections

- [[Heine–Borel Theorem]] — compact subsets of $\mathbb{R}^n$ are complete (as closed bounded sets), so Baire applies to $[a,b]$ and all of $\mathbb{R}^n$.
- [[Bolzano–Weierstrass Theorem]] — completeness of $\mathbb{R}^n$ underlies both: Bolzano–Weierstrass gives sequential compactness, Baire gives category density.
- [[Intermediate Value Theorem]] — a corollary-style consequence: continuous functions on $[a,b]$ cannot be everywhere-differentiable with derivative identically zero except on a meager set.
- [[Urysohn's Lemma]] — Urysohn's construction works in normal spaces; Baire's theorem is the key to the open-mapping theorem in Banach spaces, which are complete metric spaces.
- [[Uniform Boundedness Principle]] — the Banach–Steinhaus theorem is proved by applying Baire to the complete metric space structure of Banach spaces.
- [[Open Mapping Theorem (Banach)]] — a surjective bounded linear map between Banach spaces is open; the proof partitions the codomain into meager sets and invokes Baire.
- [[Closed Graph Theorem]] — a linear map between Banach spaces with a closed graph is bounded; the proof is a direct application of the Open Mapping Theorem, itself founded on Baire.

## Lean4 Proof

```lean4
import Mathlib.Topology.Baire.CompleteMetrizable
import Mathlib.Topology.Baire.Lemmas

/-- **Baire Category Theorem**: in a complete metric space, the intersection of
    countably many dense open sets is dense.
    Mathlib instance: `BaireSpace.of_completelyPseudoMetrizable` promotes any
    completely metrizable space to a `BaireSpace`, then
    `dense_iInter_of_isOpen_nat` gives the conclusion. -/
theorem baire_category_theorem
    {X : Type*} [TopologicalSpace X] [CompleteSpace X] [PseudoMetricSpace X]
    (U : ℕ → Set X) (hopen : ∀ n, IsOpen (U n)) (hdense : ∀ n, Dense (U n)) :
    Dense (⋂ n, U n) := by
  haveI : IsCompletelyPseudoMetrizableSpace X := ⟨⟨inferInstance, inferInstance, rfl⟩⟩
  haveI : BaireSpace X := BaireSpace.of_completelyPseudoMetrizable
  exact dense_iInter_of_isOpen_nat hopen hdense
```
